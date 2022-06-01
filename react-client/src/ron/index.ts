import { buildLexer, expectEOF } from "typescript-parsec";
import { RonDecoder, RonToken, StringLiteral } from "./common";
import { string } from "./string";
import { number } from "./number";
import { ronEnum } from "./enum";
import { tupleStruct, TupleStruct } from "./tupleStruct";

export { string, number, tupleStruct, ronEnum };
export type { TupleStruct, StringLiteral };

export type Result<V> =
    | {
          success: true;
          value: V;
      }
    | {
          success: false;
          error: string;
      };

const RON_LEXER = buildLexer([
    [true, /^[a-zA-Z][a-zA-Z\d]*/g, RonToken.Identifier],
    [true, /^"[^"]*"/g, RonToken.String],
    [true, /^\s+/g, RonToken.Whitespace],
    [true, /^,/g, RonToken.Comma],
    [true, /^\(/g, RonToken.LeftParenthesis],
    [true, /^\)/g, RonToken.RightParenthesis],
]);

export function decode<V>(input: string, decoder: RonDecoder<V>): Result<V> {
    let token;
    try {
        token = RON_LEXER.parse(input);
    } catch (e) {
        return {
            success: false,
            error: e instanceof Error ? e.message : "could not lex RON input",
        };
    }
    const parseResult = expectEOF(decoder(token));

    if (!parseResult.successful) {
        return { success: false, error: parseResult.error.message };
    }
    if (parseResult.candidates.length === 0) {
        return {
            success: false,
            error: "Decoding RON value returned no result",
        };
    }
    if (parseResult.candidates.length > 1) {
        return {
            success: false,
            error: "Decoding RON value returned ambiguous values",
        };
    }
    return { success: true, value: parseResult.candidates[0].result };
}
