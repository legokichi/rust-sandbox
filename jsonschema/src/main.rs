#[derive(serde::Deserialize, serde::Serialize, Debug, validator::Validate)]
struct A {
    #[validate(custom(function = "validate_settings", arg = "&'v_a serde_json::Value"))]
    settings: serde_json::Value,
}
fn validate_settings(
    settings: &serde_json::Value,
    arg: &serde_json::Value,
) -> Result<(), validator::ValidationError> {
    let compiled = jsonschema::JSONSchema::compile(&arg).expect("A valid schema");
    let result = compiled.validate(&settings);
    if let Err(errors) = result {
        let mut err = validator::ValidationError::new("invalid settings");
        for error in errors {
            err.add_param("error".into(), &format!("{error}"));
            err.add_param("error".into(), &format!("{error:?}"));
            err.add_param("instance path".into(), &format!("{}", error.instance_path));
            err.add_param("schema path".into(), &format!("{}", error.schema_path));
        }
        return Err(err);
    }
    Ok(())
}
fn validate_json(schema: &serde_json::Value, settings: &serde_json::Value) -> Result<(), anyhow::Error> {
    let mut scope = valico::json_schema::Scope::new();
    let compiled = scope
        .compile_and_return(schema.clone(), false)?;
    let ret = compiled.validate(settings);
    dbg!(&ret);
    if !ret.is_valid() {
        let e = anyhow::anyhow!(
            "error: {}\nschema: {}\nsettings: {}",
            serde_json::to_string_pretty(&ret.errors).expect("serializing error failed"),
            serde_json::to_string_pretty(&schema).expect("serializing schema failed"),
            serde_json::to_string_pretty(&settings).expect("serializing settings failed"),
        );
        return Err(e.into());
    }
    Ok(())
}


fn main() {
    let valid_schema = serde_json::from_str(
r##"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "properties": {
        "aws_access_key_id": {
            "default": "",
            "description": "aws_access_key_id",
            "title": "aws_access_key_id",
            "type": "string"
        },
        "aws_secret_access_key": {
            "default": "",
            "description": "aws_secret_access_key",
            "title": "aws_secret_access_key",
            "type": "string"
        },
        "capture_framerate": {
            "default": 10,
            "description": "Capture framerate (enable if resize mode)",
            "minimum": 5,
            "title": "capture framerate",
            "type": "integer"
        },
        "capture_size": {
            "default": "640x480",
            "description": "scale of capture image size",
            "title": "[Capture] capture size",
            "type": "string"
        },
        "crop_area": {
            "default": "0.0,0.0,1.0,1.0",
            "description": "perform cropping against a rectangular area on the captured images defined as: left = crop_area[0] * capture_size[0], top = crop_area[1] * capture_size[1], right = crop_area[2] * capture_size[0], bottom = crop_area[3] * capture_size[1]. Each element of crop_area must be in the range of [0.0, 1.0].",
            "title": "crop area",
            "type": "string"
        },
        "crop_mode": {
            "default": "center",
            "description": "crop_mode",
            "title": "crop mode",
            "type": "string"
        },
        "detect_mintime": {
            "default": 0.5,
            "description": "detect minimum time",
            "minimum": 0,
            "title": "detect minimum time",
            "type": "number"
        },
        "detection_area": {
            "default": "0.0,0.0,1.0,1.0",
            "description": "List of rectangles to be used as detection areas. Each rectangle is specified by (left, up, right, bottom). Each coordinate is specified by [0, 1]. The format of the specification is a sequence of coordinates l0,u0,r0,b0,l1,u1,r1,b1,... The format of the specification is described as a sequence of coordinates [0, 1].",
            "title": "detection area mask",
            "type": "string"
        },
        "detection_area_margin": {
            "default": 0.01,
            "description": "detection_area_margin",
            "maximum": 1,
            "minimum": 0,
            "title": "detection_area_margin (obsolete)",
            "type": "number"
        },
        "display": {
            "default": true,
            "description": "output video to HDMI display",
            "title": "display",
            "type": "boolean"
        },
        "exposure_time": {
            "default": 0,
            "description": "selects the exposure time of the camera (0: auto)",
            "title": "exposure time",
            "type": "integer"
        },
        "faceconf_moving_average_weight": {
            "default": 1,
            "description": "weight for moving average of face confidence score. 1.0 means not taking the average.",
            "maximum": 1,
            "minimum": 0,
            "title": "weight for moving average of face confidence score.",
            "type": "number"
        },
        "faceid_waittime": {
            "default": 1.5,
            "description": "time, in seconds, to wait before assigning Face ID in detection area",
            "minimum": 0,
            "title": "FaceID wait time",
            "type": "number"
        },
        "facepose_moving_average_weight": {
            "default": 1,
            "description": "weight for moving average of face pose. 1.0 means not taking the average.",
            "maximum": 1,
            "minimum": 0,
            "title": "weight for moving average of face pose.",
            "type": "number"
        },
        "frame_in_count": {
            "default": false,
            "description": "frame-in count",
            "title": "frame-in count",
            "type": "boolean"
        },
        "hflip": {
            "default": false,
            "description": "flip camera capture",
            "title": "horizontal flip camera",
            "type": "boolean"
        },
        "max_frontal_list_length": {
            "default": 1000,
            "description": "maximum length allowed for the frontal_list, 0 to unlimited",
            "minimum": 0,
            "title": "max frontal_list length",
            "type": "integer"
        },
        "maximum_bbox_size": {
            "default": "256x256",
            "description": "Maximum size of bboxes",
            "title": "Maximum size of bboxes",
            "type": "string"
        },
        "maximum_pitch": {
            "default": 10,
            "description": "threshold for detection of frontalness (in degrees). The face is frontal if pitch are below the maxinum threshold",
            "title": "maxinum pitch",
            "type": "number"
        },
        "maximum_roll": {
            "default": 10,
            "description": "threshold for detection of frontalness (in degrees). The face is frontal if roll are below the maxinum threshold",
            "title": "maxinum roll",
            "type": "number"
        },
        "maximum_yaw": {
            "default": 10,
            "description": "threshold for detection of frontalness (in degrees). The face is frontal if yaw are below the maxinum threshold",
            "title": "maxinum yaw",
            "type": "number"
        },
        "maxmisscount": {
            "default": 10,
            "description": "maxmisscount",
            "minimum": 0,
            "title": "maxmisscount",
            "type": "integer"
        },
        "minimum_bbox_size": {
            "default": "5x5",
            "description": "Minimum size of bboxes",
            "title": "Minimum size of bboxes",
            "type": "string"
        },
        "minimum_pitch": {
            "default": -10,
            "description": "threshold for detection of frontalness (in degrees). The face is frontal if pitch are below the minimum threshold",
            "title": "minimum pitch",
            "type": "number"
        },
        "minimum_roll": {
            "default": -10,
            "description": "threshold for detection of frontalness (in degrees). The face is frontal if roll are below the minimum threshold",
            "title": "minimum roll",
            "type": "number"
        },
        "minimum_yaw": {
            "default": -10,
            "description": "threshold for detection of frontalness (in degrees). The face is frontal if yaw are below the minimum threshold",
            "title": "minimum yaw",
            "type": "number"
        },
        "nms_thresh": {
            "default": 0.3,
            "description": "nms threshold for detection by SSD",
            "maximum": 1,
            "minimum": 0,
            "title": "nms threshold",
            "type": "number"
        },
        "out_non_frontal_face": {
            "default": true,
            "description": "cast information for no detect frontal face.",
            "title": "cast no frontal face",
            "type": "boolean"
        },
        "patch_top_margin": {
            "default": 0.1,
            "description": "Margin for top-side of face patch",
            "maximum": 1,
            "minimum": 0,
            "title": "Margin for top-side of face patch",
            "type": "number"
        },
        "person_score_thresh": {
            "default": 0.2,
            "description": "score threshold for person detection by YOLO",
            "maximum": 1,
            "minimum": 0,
            "title": "person score threshold",
            "type": "number"
        },
        "preview_size": {
            "default": "",
            "description": "preview_size",
            "title": "preview_size",
            "type": "string"
        },
        "region_name": {
            "default": "",
            "description": "region name",
            "title": "region name",
            "type": "string"
        },
        "reid_maxinterval": {
            "default": 300,
            "description": "Maximum interval (sec) to perform Face ReID",
            "minimum": 0,
            "title": "ReID interval",
            "type": "number"
        },
        "reid_thresh": {
            "default": 0.8,
            "description": "Threshold for Face ReID",
            "maximum": 1,
            "minimum": 0,
            "title": "ReID threshold",
            "type": "number"
        },
        "reid_trigger_thresh": {
            "default": 0.8,
            "description": "Threshold for face confidence for trigerring Face ReID",
            "maximum": 1,
            "minimum": 0,
            "title": "ReID trigger threshold",
            "type": "number"
        },
        "rotation": {
            "default": 0,
            "description": "rotate camera capture (0 or 90 or -90 or 180)",
            "title": "camera rotation",
            "type": "integer"
        },
        "score_thresh": {
            "default": 0.4,
            "description": "score threshold for detection by YOLO",
            "maximum": 1,
            "minimum": 0,
            "title": "score threshold",
            "type": "number"
        },
        "score_thresh_second_stage": {
            "default": 0.2,
            "description": "score threshold for detection by face pose model",
            "maximum": 1,
            "minimum": 0,
            "title": "score threshold for second stage",
            "type": "number"
        },
        "score_thresh_third_stage": {
            "default": 0.4,
            "description": "score threshold for face classifier",
            "maximum": 1,
            "minimum": 0,
            "title": "score threshold for third stage",
            "type": "number"
        },
        "sendcapture_framethreshold": {
            "default": 3,
            "description": "send capture image when frame count more than this threshold",
            "minimum": 0,
            "title": "sendcapture frame threshold",
            "type": "integer"
        },
        "sendcapture_interval": {
            "default": 10,
            "description": "send capture image from before send image",
            "minimum": 0,
            "title": "sendcapture interval",
            "type": "number"
        },
        "sendcapture_prefix": {
            "default": "",
            "description": "sendcapture prefix",
            "title": "sendcapture prefix",
            "type": "string"
        },
        "sendcapture_queuesize": {
            "default": 1,
            "description": "sendcapture queue size",
            "minimum": 0,
            "title": "sendcapture queue size",
            "type": "integer"
        },
        "setting_mode": {
            "default": false,
            "description": "setting_mode",
            "title": "setting_mode",
            "type": "boolean"
        },
        "show_agegender": {
            "default": true,
            "description": "show age and gender.",
            "title": "show age gender",
            "type": "boolean"
        },
        "show_bbox": {
            "default": true,
            "description": "show face bboxes. Even if disabled, the pose projection centered on the face is still displayed.",
            "title": "show face bboxes",
            "type": "boolean"
        },
        "show_count": {
            "default": false,
            "description": "show count of people in the output video",
            "title": "show count of people",
            "type": "boolean"
        },
        "show_datetime": {
            "default": true,
            "description": "show datetime on display",
            "title": "show datetime on display",
            "type": "boolean"
        },
        "show_faceid_log": {
            "default": true,
            "description": "show log of face ID",
            "title": "show log of face ID",
            "type": "boolean"
        },
        "show_fps": {
            "default": true,
            "description": "show FPS in the output video",
            "title": "show fps on display",
            "type": "boolean"
        },
        "show_frontal_time": {
            "default": true,
            "description": "show frontal time in seconds.",
            "title": "show frontal time",
            "type": "boolean"
        },
        "show_params": {
            "default": true,
            "description": "show parameters of this act.",
            "title": "show parameters of this act",
            "type": "boolean"
        },
        "show_posebox": {
            "default": true,
            "description": "show pose box.",
            "title": "show pose box",
            "type": "boolean"
        },
        "show_posevalue": {
            "default": false,
            "description": "show pose value.",
            "title": "show pose value",
            "type": "boolean"
        },
        "show_probability": {
            "default": false,
            "description": "show probability of detected object.",
            "title": "show probability",
            "type": "boolean"
        },
        "show_trackevent": {
            "default": true,
            "description": "show tracking event (ReID of BBOX).",
            "title": "show tracking event (ReID or BBOX)",
            "type": "boolean"
        },
        "show_trackid": {
            "default": true,
            "description": "show tracking id of faces.",
            "title": "show tracking id",
            "type": "boolean"
        },
        "show_trail": {
            "default": true,
            "description": "show trails of detected heads.",
            "title": "show trails of heads",
            "type": "boolean"
        },
        "stream_name": {
            "default": "",
            "description": "stream name",
            "title": "stream name",
            "type": "string"
        },
        "tracker_bbox_margin": {
            "default": 1,
            "description": "margin for bboxes when computing IoU with other boxes for tracking",
            "title": "tracker bbox margin",
            "type": "number"
        },
        "tracker_iou_weight": {
            "default": 0.5,
            "description": "weight for IoU score",
            "maximum": 1,
            "minimum": 0,
            "title": "tracker iou weight",
            "type": "number"
        },
        "tracker_thresh": {
            "default": 0.1,
            "description": "threshold for tracking bboxes",
            "maximum": 1,
            "minimum": 0,
            "title": "tracker threshold",
            "type": "number"
        }
    },
    "propertyOrder": [
        "display",
        "rotation",
        "exposure_time",
        "hflip",
        "nms_thresh",
        "person_score_thresh",
        "score_thresh",
        "score_thresh_second_stage",
        "score_thresh_third_stage",
        "detect_mintime",
        "out_non_frontal_face",
        "show_datetime",
        "show_fps",
        "show_count",
        "show_bbox",
        "show_agegender",
        "show_posevalue",
        "show_posebox",
        "show_trackid",
        "show_trackevent",
        "show_frontal_time",
        "show_probability",
        "show_params",
        "show_trail",
        "show_faceid_log",
        "capture_size",
        "capture_framerate",
        "maximum_pitch",
        "minimum_pitch",
        "maximum_yaw",
        "minimum_yaw",
        "maximum_roll",
        "minimum_roll",
        "tracker_bbox_margin",
        "tracker_thresh",
        "tracker_iou_weight",
        "faceconf_moving_average_weight",
        "facepose_moving_average_weight",
        "minimum_bbox_size",
        "maximum_bbox_size",
        "patch_top_margin",
        "maxmisscount",
        "reid_trigger_thresh",
        "reid_thresh",
        "reid_maxinterval",
        "faceid_waittime",
        "frame_in_count",
        "detection_area_margin",
        "detection_area",
        "crop_mode",
        "crop_area",
        "sendcapture_prefix",
        "sendcapture_interval",
        "sendcapture_framethreshold",
        "sendcapture_queuesize",
        "stream_name",
        "region_name",
        "aws_access_key_id",
        "aws_secret_access_key",
        "preview_size",
        "max_frontal_list_length",
        "setting_mode"
    ],
    "required": [
        "aws_access_key_id",
        "aws_secret_access_key",
        "capture_framerate",
        "capture_size",
        "crop_area",
        "crop_mode",
        "detect_mintime",
        "detection_area_margin",
        "display",
        "exposure_time",
        "hflip",
        "maximum_pitch",
        "maximum_roll",
        "maximum_yaw",
        "tracker_bbox_margin",
        "tracker_thresh",
        "tracker_iou_weight",
        "faceconf_moving_average_weight",
        "facepose_moving_average_weight",
        "minimum_bbox_size",
        "maximum_bbox_size",
        "patch_top_margin",
        "maxmisscount",
        "minimum_pitch",
        "minimum_roll",
        "minimum_yaw",
        "out_non_frontal_face",
        "preview_size",
        "region_name",
        "rotation",
        "nms_thresh",
        "person_score_thresh",
        "score_thresh",
        "score_thresh_second_stage",
        "score_thresh_third_stage",
        "frame_in_count",
        "sendcapture_framethreshold",
        "sendcapture_interval",
        "sendcapture_prefix",
        "sendcapture_queuesize",
        "show_agegender",
        "show_bbox",
        "show_datetime",
        "show_fps",
        "show_count",
        "show_posebox",
        "show_posevalue",
        "show_trackid",
        "show_trackevent",
        "show_frontal_time",
        "show_probability",
        "show_params",
        "show_trail",
        "show_faceid_log",
        "stream_name",
        "setting_mode"
    ],
    "type": "object"
}"##,
    )
    .unwrap();
    let valid_json = serde_json::from_str(
        r#"[{ 
              "display": "Bool(false)"
            }]"#,
    )
    .unwrap();
    let o = validate_json(&valid_schema, &valid_json);
    assert!(o.is_ok());



    let schema = include_str!("schema.json");
    let schema = serde_json::from_str::<serde_json::Value>(schema).unwrap();
    let schema = schema
        .pointer("/definitions/AWSLambda.KinesisStreamRecord")
        .unwrap();
    println!("{}",serde_json::to_string_pretty(schema).unwrap());
    let settings = serde_json::json!({});
    let err = validate_json(&settings, schema).unwrap_err();

    // let a = serde_json::json!({"settings":{}});
    // let a = serde_json::from_value::<A>(a).unwrap();
    // use validator::ValidateArgs;
    // let err = a.validate_args(&schema).unwrap_err();
    println!("{err}");
    println!("{err:?}");
}
