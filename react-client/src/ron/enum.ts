import { RonDecoder, RonToken } from "./common";
import { ParserOutput, Token } from "typescript-parsec";
import { TupleStruct } from "./tupleStruct";

export function ronEnum<Variants extends [...NamedValue[]]>(
    ...variantDecoders: EnumVariantDecoders<Variants>
): RonDecoder<EnumVariants<Variants>> {
    return (token: Token<RonToken> | undefined): ParserOutput<RonToken, EnumVariants<Variants>> => {
        return decodeEnum(token, variantDecoders);
    };
}

function decodeEnum<Variants extends [...NamedValue[]]>(
    token: Token<RonToken> | undefined,
    variantDecoders: EnumVariantDecoders<Variants>
): ParserOutput<RonToken, EnumVariants<Variants>> {
    if (variantDecoders.length === 0) {
        return {
            successful: false,
            error: {
                kind: "Error",
                pos: undefined,
                message: 'Did not find matching enum variant for "' + reassembleTokens(token) + '"',
            },
        };
    }

    const [head, ...tail] = variantDecoders;
    let result = (head as RonDecoder<Variants[0]>)(token);
    if (result.successful) {
        return result as unknown as ParserOutput<RonToken, EnumVariants<Variants>>;
    }

    return decodeEnum(token, tail);
}

function reassembleTokens(token: Token<RonToken> | undefined): string {
    if (token === undefined) {
        return "";
    }
    return token.text + reassembleTokens(token.next);
}

type EnumVariants<Variants extends NamedValue[]> = Variants extends [
    TupleStruct<any, any>,
    ...infer Tail
]
    ? Tail extends NamedValue[]
        ? Variants[0] | EnumVariants<Tail>
        : "Enum variant tail invalid"
    : never;

type EnumVariantDecoders<V extends [...NamedValue[]]> = {
    [Key in keyof V]: V[Key] extends TupleStruct<any, any> ? RonDecoder<V[Key]> : never;
};

type NamedValue = { name: string; value: any };
