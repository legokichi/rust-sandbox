/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
"use strict";

var $protobuf = require("protobufjs/minimal");

// Common aliases
var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

$root.example = (function() {

    /**
     * Namespace example.
     * @exports example
     * @namespace
     */
    var example = {};

    example.protobuf = (function() {

        /**
         * Namespace protobuf.
         * @memberof example
         * @namespace
         */
        var protobuf = {};

        protobuf.SimpleMessage = (function() {

            /**
             * Properties of a SimpleMessage.
             * @memberof example.protobuf
             * @interface ISimpleMessage
             * @property {number|Long|null} [id] SimpleMessage id
             * @property {example.protobuf.SimpleMessage.Type|null} [messageType] SimpleMessage messageType
             * @property {Array.<example.protobuf.SimpleMessage.IHeaderItem>|null} [header] SimpleMessage header
             * @property {Uint8Array|null} [blob] SimpleMessage blob
             * @property {string|null} [plaintext] SimpleMessage plaintext
             */

            /**
             * Constructs a new SimpleMessage.
             * @memberof example.protobuf
             * @classdesc Represents a SimpleMessage.
             * @implements ISimpleMessage
             * @constructor
             * @param {example.protobuf.ISimpleMessage=} [properties] Properties to set
             */
            function SimpleMessage(properties) {
                this.header = [];
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * SimpleMessage id.
             * @member {number|Long} id
             * @memberof example.protobuf.SimpleMessage
             * @instance
             */
            SimpleMessage.prototype.id = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

            /**
             * SimpleMessage messageType.
             * @member {example.protobuf.SimpleMessage.Type} messageType
             * @memberof example.protobuf.SimpleMessage
             * @instance
             */
            SimpleMessage.prototype.messageType = 0;

            /**
             * SimpleMessage header.
             * @member {Array.<example.protobuf.SimpleMessage.IHeaderItem>} header
             * @memberof example.protobuf.SimpleMessage
             * @instance
             */
            SimpleMessage.prototype.header = $util.emptyArray;

            /**
             * SimpleMessage blob.
             * @member {Uint8Array} blob
             * @memberof example.protobuf.SimpleMessage
             * @instance
             */
            SimpleMessage.prototype.blob = $util.newBuffer([]);

            /**
             * SimpleMessage plaintext.
             * @member {string} plaintext
             * @memberof example.protobuf.SimpleMessage
             * @instance
             */
            SimpleMessage.prototype.plaintext = "";

            // OneOf field names bound to virtual getters and setters
            var $oneOfFields;

            /**
             * SimpleMessage data.
             * @member {"blob"|"plaintext"|undefined} data
             * @memberof example.protobuf.SimpleMessage
             * @instance
             */
            Object.defineProperty(SimpleMessage.prototype, "data", {
                get: $util.oneOfGetter($oneOfFields = ["blob", "plaintext"]),
                set: $util.oneOfSetter($oneOfFields)
            });

            /**
             * Creates a new SimpleMessage instance using the specified properties.
             * @function create
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {example.protobuf.ISimpleMessage=} [properties] Properties to set
             * @returns {example.protobuf.SimpleMessage} SimpleMessage instance
             */
            SimpleMessage.create = function create(properties) {
                return new SimpleMessage(properties);
            };

            /**
             * Encodes the specified SimpleMessage message. Does not implicitly {@link example.protobuf.SimpleMessage.verify|verify} messages.
             * @function encode
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {example.protobuf.ISimpleMessage} message SimpleMessage message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SimpleMessage.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.id != null && Object.hasOwnProperty.call(message, "id"))
                    writer.uint32(/* id 1, wireType 0 =*/8).uint64(message.id);
                if (message.messageType != null && Object.hasOwnProperty.call(message, "messageType"))
                    writer.uint32(/* id 2, wireType 0 =*/16).int32(message.messageType);
                if (message.header != null && message.header.length)
                    for (var i = 0; i < message.header.length; ++i)
                        $root.example.protobuf.SimpleMessage.HeaderItem.encode(message.header[i], writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.blob != null && Object.hasOwnProperty.call(message, "blob"))
                    writer.uint32(/* id 4, wireType 2 =*/34).bytes(message.blob);
                if (message.plaintext != null && Object.hasOwnProperty.call(message, "plaintext"))
                    writer.uint32(/* id 5, wireType 2 =*/42).string(message.plaintext);
                return writer;
            };

            /**
             * Encodes the specified SimpleMessage message, length delimited. Does not implicitly {@link example.protobuf.SimpleMessage.verify|verify} messages.
             * @function encodeDelimited
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {example.protobuf.ISimpleMessage} message SimpleMessage message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SimpleMessage.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a SimpleMessage message from the specified reader or buffer.
             * @function decode
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {example.protobuf.SimpleMessage} SimpleMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SimpleMessage.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.example.protobuf.SimpleMessage();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.id = reader.uint64();
                        break;
                    case 2:
                        message.messageType = reader.int32();
                        break;
                    case 3:
                        if (!(message.header && message.header.length))
                            message.header = [];
                        message.header.push($root.example.protobuf.SimpleMessage.HeaderItem.decode(reader, reader.uint32()));
                        break;
                    case 4:
                        message.blob = reader.bytes();
                        break;
                    case 5:
                        message.plaintext = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a SimpleMessage message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {example.protobuf.SimpleMessage} SimpleMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SimpleMessage.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a SimpleMessage message.
             * @function verify
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            SimpleMessage.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                var properties = {};
                if (message.id != null && message.hasOwnProperty("id"))
                    if (!$util.isInteger(message.id) && !(message.id && $util.isInteger(message.id.low) && $util.isInteger(message.id.high)))
                        return "id: integer|Long expected";
                if (message.messageType != null && message.hasOwnProperty("messageType"))
                    switch (message.messageType) {
                    default:
                        return "messageType: enum value expected";
                    case 0:
                    case 1:
                    case 2:
                        break;
                    }
                if (message.header != null && message.hasOwnProperty("header")) {
                    if (!Array.isArray(message.header))
                        return "header: array expected";
                    for (var i = 0; i < message.header.length; ++i) {
                        var error = $root.example.protobuf.SimpleMessage.HeaderItem.verify(message.header[i]);
                        if (error)
                            return "header." + error;
                    }
                }
                if (message.blob != null && message.hasOwnProperty("blob")) {
                    properties.data = 1;
                    if (!(message.blob && typeof message.blob.length === "number" || $util.isString(message.blob)))
                        return "blob: buffer expected";
                }
                if (message.plaintext != null && message.hasOwnProperty("plaintext")) {
                    if (properties.data === 1)
                        return "data: multiple values";
                    properties.data = 1;
                    if (!$util.isString(message.plaintext))
                        return "plaintext: string expected";
                }
                return null;
            };

            /**
             * Creates a SimpleMessage message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {example.protobuf.SimpleMessage} SimpleMessage
             */
            SimpleMessage.fromObject = function fromObject(object) {
                if (object instanceof $root.example.protobuf.SimpleMessage)
                    return object;
                var message = new $root.example.protobuf.SimpleMessage();
                if (object.id != null)
                    if ($util.Long)
                        (message.id = $util.Long.fromValue(object.id)).unsigned = true;
                    else if (typeof object.id === "string")
                        message.id = parseInt(object.id, 10);
                    else if (typeof object.id === "number")
                        message.id = object.id;
                    else if (typeof object.id === "object")
                        message.id = new $util.LongBits(object.id.low >>> 0, object.id.high >>> 0).toNumber(true);
                switch (object.messageType) {
                case "START":
                case 0:
                    message.messageType = 0;
                    break;
                case "BLOB":
                case 1:
                    message.messageType = 1;
                    break;
                case "END":
                case 2:
                    message.messageType = 2;
                    break;
                }
                if (object.header) {
                    if (!Array.isArray(object.header))
                        throw TypeError(".example.protobuf.SimpleMessage.header: array expected");
                    message.header = [];
                    for (var i = 0; i < object.header.length; ++i) {
                        if (typeof object.header[i] !== "object")
                            throw TypeError(".example.protobuf.SimpleMessage.header: object expected");
                        message.header[i] = $root.example.protobuf.SimpleMessage.HeaderItem.fromObject(object.header[i]);
                    }
                }
                if (object.blob != null)
                    if (typeof object.blob === "string")
                        $util.base64.decode(object.blob, message.blob = $util.newBuffer($util.base64.length(object.blob)), 0);
                    else if (object.blob.length)
                        message.blob = object.blob;
                if (object.plaintext != null)
                    message.plaintext = String(object.plaintext);
                return message;
            };

            /**
             * Creates a plain object from a SimpleMessage message. Also converts values to other types if specified.
             * @function toObject
             * @memberof example.protobuf.SimpleMessage
             * @static
             * @param {example.protobuf.SimpleMessage} message SimpleMessage
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            SimpleMessage.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.arrays || options.defaults)
                    object.header = [];
                if (options.defaults) {
                    if ($util.Long) {
                        var long = new $util.Long(0, 0, true);
                        object.id = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                    } else
                        object.id = options.longs === String ? "0" : 0;
                    object.messageType = options.enums === String ? "START" : 0;
                }
                if (message.id != null && message.hasOwnProperty("id"))
                    if (typeof message.id === "number")
                        object.id = options.longs === String ? String(message.id) : message.id;
                    else
                        object.id = options.longs === String ? $util.Long.prototype.toString.call(message.id) : options.longs === Number ? new $util.LongBits(message.id.low >>> 0, message.id.high >>> 0).toNumber(true) : message.id;
                if (message.messageType != null && message.hasOwnProperty("messageType"))
                    object.messageType = options.enums === String ? $root.example.protobuf.SimpleMessage.Type[message.messageType] : message.messageType;
                if (message.header && message.header.length) {
                    object.header = [];
                    for (var j = 0; j < message.header.length; ++j)
                        object.header[j] = $root.example.protobuf.SimpleMessage.HeaderItem.toObject(message.header[j], options);
                }
                if (message.blob != null && message.hasOwnProperty("blob")) {
                    object.blob = options.bytes === String ? $util.base64.encode(message.blob, 0, message.blob.length) : options.bytes === Array ? Array.prototype.slice.call(message.blob) : message.blob;
                    if (options.oneofs)
                        object.data = "blob";
                }
                if (message.plaintext != null && message.hasOwnProperty("plaintext")) {
                    object.plaintext = message.plaintext;
                    if (options.oneofs)
                        object.data = "plaintext";
                }
                return object;
            };

            /**
             * Converts this SimpleMessage to JSON.
             * @function toJSON
             * @memberof example.protobuf.SimpleMessage
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            SimpleMessage.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            SimpleMessage.HeaderItem = (function() {

                /**
                 * Properties of a HeaderItem.
                 * @memberof example.protobuf.SimpleMessage
                 * @interface IHeaderItem
                 * @property {string|null} [name] HeaderItem name
                 * @property {string|null} [value] HeaderItem value
                 */

                /**
                 * Constructs a new HeaderItem.
                 * @memberof example.protobuf.SimpleMessage
                 * @classdesc Represents a HeaderItem.
                 * @implements IHeaderItem
                 * @constructor
                 * @param {example.protobuf.SimpleMessage.IHeaderItem=} [properties] Properties to set
                 */
                function HeaderItem(properties) {
                    if (properties)
                        for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                            if (properties[keys[i]] != null)
                                this[keys[i]] = properties[keys[i]];
                }

                /**
                 * HeaderItem name.
                 * @member {string} name
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @instance
                 */
                HeaderItem.prototype.name = "";

                /**
                 * HeaderItem value.
                 * @member {string} value
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @instance
                 */
                HeaderItem.prototype.value = "";

                /**
                 * Creates a new HeaderItem instance using the specified properties.
                 * @function create
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {example.protobuf.SimpleMessage.IHeaderItem=} [properties] Properties to set
                 * @returns {example.protobuf.SimpleMessage.HeaderItem} HeaderItem instance
                 */
                HeaderItem.create = function create(properties) {
                    return new HeaderItem(properties);
                };

                /**
                 * Encodes the specified HeaderItem message. Does not implicitly {@link example.protobuf.SimpleMessage.HeaderItem.verify|verify} messages.
                 * @function encode
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {example.protobuf.SimpleMessage.IHeaderItem} message HeaderItem message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                HeaderItem.encode = function encode(message, writer) {
                    if (!writer)
                        writer = $Writer.create();
                    if (message.name != null && Object.hasOwnProperty.call(message, "name"))
                        writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
                    if (message.value != null && Object.hasOwnProperty.call(message, "value"))
                        writer.uint32(/* id 2, wireType 2 =*/18).string(message.value);
                    return writer;
                };

                /**
                 * Encodes the specified HeaderItem message, length delimited. Does not implicitly {@link example.protobuf.SimpleMessage.HeaderItem.verify|verify} messages.
                 * @function encodeDelimited
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {example.protobuf.SimpleMessage.IHeaderItem} message HeaderItem message or plain object to encode
                 * @param {$protobuf.Writer} [writer] Writer to encode to
                 * @returns {$protobuf.Writer} Writer
                 */
                HeaderItem.encodeDelimited = function encodeDelimited(message, writer) {
                    return this.encode(message, writer).ldelim();
                };

                /**
                 * Decodes a HeaderItem message from the specified reader or buffer.
                 * @function decode
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @param {number} [length] Message length if known beforehand
                 * @returns {example.protobuf.SimpleMessage.HeaderItem} HeaderItem
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                HeaderItem.decode = function decode(reader, length) {
                    if (!(reader instanceof $Reader))
                        reader = $Reader.create(reader);
                    var end = length === undefined ? reader.len : reader.pos + length, message = new $root.example.protobuf.SimpleMessage.HeaderItem();
                    while (reader.pos < end) {
                        var tag = reader.uint32();
                        switch (tag >>> 3) {
                        case 1:
                            message.name = reader.string();
                            break;
                        case 2:
                            message.value = reader.string();
                            break;
                        default:
                            reader.skipType(tag & 7);
                            break;
                        }
                    }
                    return message;
                };

                /**
                 * Decodes a HeaderItem message from the specified reader or buffer, length delimited.
                 * @function decodeDelimited
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
                 * @returns {example.protobuf.SimpleMessage.HeaderItem} HeaderItem
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                HeaderItem.decodeDelimited = function decodeDelimited(reader) {
                    if (!(reader instanceof $Reader))
                        reader = new $Reader(reader);
                    return this.decode(reader, reader.uint32());
                };

                /**
                 * Verifies a HeaderItem message.
                 * @function verify
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {Object.<string,*>} message Plain object to verify
                 * @returns {string|null} `null` if valid, otherwise the reason why it is not
                 */
                HeaderItem.verify = function verify(message) {
                    if (typeof message !== "object" || message === null)
                        return "object expected";
                    if (message.name != null && message.hasOwnProperty("name"))
                        if (!$util.isString(message.name))
                            return "name: string expected";
                    if (message.value != null && message.hasOwnProperty("value"))
                        if (!$util.isString(message.value))
                            return "value: string expected";
                    return null;
                };

                /**
                 * Creates a HeaderItem message from a plain object. Also converts values to their respective internal types.
                 * @function fromObject
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {Object.<string,*>} object Plain object
                 * @returns {example.protobuf.SimpleMessage.HeaderItem} HeaderItem
                 */
                HeaderItem.fromObject = function fromObject(object) {
                    if (object instanceof $root.example.protobuf.SimpleMessage.HeaderItem)
                        return object;
                    var message = new $root.example.protobuf.SimpleMessage.HeaderItem();
                    if (object.name != null)
                        message.name = String(object.name);
                    if (object.value != null)
                        message.value = String(object.value);
                    return message;
                };

                /**
                 * Creates a plain object from a HeaderItem message. Also converts values to other types if specified.
                 * @function toObject
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @static
                 * @param {example.protobuf.SimpleMessage.HeaderItem} message HeaderItem
                 * @param {$protobuf.IConversionOptions} [options] Conversion options
                 * @returns {Object.<string,*>} Plain object
                 */
                HeaderItem.toObject = function toObject(message, options) {
                    if (!options)
                        options = {};
                    var object = {};
                    if (options.defaults) {
                        object.name = "";
                        object.value = "";
                    }
                    if (message.name != null && message.hasOwnProperty("name"))
                        object.name = message.name;
                    if (message.value != null && message.hasOwnProperty("value"))
                        object.value = message.value;
                    return object;
                };

                /**
                 * Converts this HeaderItem to JSON.
                 * @function toJSON
                 * @memberof example.protobuf.SimpleMessage.HeaderItem
                 * @instance
                 * @returns {Object.<string,*>} JSON object
                 */
                HeaderItem.prototype.toJSON = function toJSON() {
                    return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
                };

                return HeaderItem;
            })();

            /**
             * Type enum.
             * @name example.protobuf.SimpleMessage.Type
             * @enum {number}
             * @property {number} START=0 START value
             * @property {number} BLOB=1 BLOB value
             * @property {number} END=2 END value
             */
            SimpleMessage.Type = (function() {
                var valuesById = {}, values = Object.create(valuesById);
                values[valuesById[0] = "START"] = 0;
                values[valuesById[1] = "BLOB"] = 1;
                values[valuesById[2] = "END"] = 2;
                return values;
            })();

            return SimpleMessage;
        })();

        return protobuf;
    })();

    return example;
})();

module.exports = $root;
