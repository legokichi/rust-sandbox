import * as $protobuf from "protobufjs";
/** Namespace example. */
export namespace example {

    /** Namespace protobuf. */
    namespace protobuf {

        /** Properties of a SimpleMessage. */
        interface ISimpleMessage {

            /** SimpleMessage id */
            id?: (number|Long|null);

            /** SimpleMessage messageType */
            messageType?: (example.protobuf.SimpleMessage.Type|null);

            /** SimpleMessage header */
            header?: (example.protobuf.SimpleMessage.IHeaderItem[]|null);

            /** SimpleMessage blob */
            blob?: (Uint8Array|null);

            /** SimpleMessage plaintext */
            plaintext?: (string|null);
        }

        /** Represents a SimpleMessage. */
        class SimpleMessage implements ISimpleMessage {

            /**
             * Constructs a new SimpleMessage.
             * @param [properties] Properties to set
             */
            constructor(properties?: example.protobuf.ISimpleMessage);

            /** SimpleMessage id. */
            public id: (number|Long);

            /** SimpleMessage messageType. */
            public messageType: example.protobuf.SimpleMessage.Type;

            /** SimpleMessage header. */
            public header: example.protobuf.SimpleMessage.IHeaderItem[];

            /** SimpleMessage blob. */
            public blob: Uint8Array;

            /** SimpleMessage plaintext. */
            public plaintext: string;

            /** SimpleMessage data. */
            public data?: ("blob"|"plaintext");

            /**
             * Creates a new SimpleMessage instance using the specified properties.
             * @param [properties] Properties to set
             * @returns SimpleMessage instance
             */
            public static create(properties?: example.protobuf.ISimpleMessage): example.protobuf.SimpleMessage;

            /**
             * Encodes the specified SimpleMessage message. Does not implicitly {@link example.protobuf.SimpleMessage.verify|verify} messages.
             * @param message SimpleMessage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: example.protobuf.ISimpleMessage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified SimpleMessage message, length delimited. Does not implicitly {@link example.protobuf.SimpleMessage.verify|verify} messages.
             * @param message SimpleMessage message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: example.protobuf.ISimpleMessage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a SimpleMessage message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns SimpleMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): example.protobuf.SimpleMessage;

            /**
             * Decodes a SimpleMessage message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns SimpleMessage
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): example.protobuf.SimpleMessage;

            /**
             * Verifies a SimpleMessage message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a SimpleMessage message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns SimpleMessage
             */
            public static fromObject(object: { [k: string]: any }): example.protobuf.SimpleMessage;

            /**
             * Creates a plain object from a SimpleMessage message. Also converts values to other types if specified.
             * @param message SimpleMessage
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: example.protobuf.SimpleMessage, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this SimpleMessage to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace SimpleMessage {

            /** Properties of a HeaderItem. */
            interface IHeaderItem {

                /** HeaderItem name */
                name?: (string|null);

                /** HeaderItem value */
                value?: (string|null);
            }

            /** Represents a HeaderItem. */
            class HeaderItem implements IHeaderItem {

                /**
                 * Constructs a new HeaderItem.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: example.protobuf.SimpleMessage.IHeaderItem);

                /** HeaderItem name. */
                public name: string;

                /** HeaderItem value. */
                public value: string;

                /**
                 * Creates a new HeaderItem instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns HeaderItem instance
                 */
                public static create(properties?: example.protobuf.SimpleMessage.IHeaderItem): example.protobuf.SimpleMessage.HeaderItem;

                /**
                 * Encodes the specified HeaderItem message. Does not implicitly {@link example.protobuf.SimpleMessage.HeaderItem.verify|verify} messages.
                 * @param message HeaderItem message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: example.protobuf.SimpleMessage.IHeaderItem, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified HeaderItem message, length delimited. Does not implicitly {@link example.protobuf.SimpleMessage.HeaderItem.verify|verify} messages.
                 * @param message HeaderItem message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: example.protobuf.SimpleMessage.IHeaderItem, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a HeaderItem message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns HeaderItem
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): example.protobuf.SimpleMessage.HeaderItem;

                /**
                 * Decodes a HeaderItem message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns HeaderItem
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): example.protobuf.SimpleMessage.HeaderItem;

                /**
                 * Verifies a HeaderItem message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a HeaderItem message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns HeaderItem
                 */
                public static fromObject(object: { [k: string]: any }): example.protobuf.SimpleMessage.HeaderItem;

                /**
                 * Creates a plain object from a HeaderItem message. Also converts values to other types if specified.
                 * @param message HeaderItem
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: example.protobuf.SimpleMessage.HeaderItem, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this HeaderItem to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Type enum. */
            enum Type {
                START = 0,
                BLOB = 1,
                END = 2
            }
        }
    }
}
