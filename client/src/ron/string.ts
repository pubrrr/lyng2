import { apply, tok, Token } from "typescript-parsec";
import { RonToken } from "./common";

export const string = apply(tok(RonToken.String), stripDoubleQuotes).parse;

function stripDoubleQuotes(token: Token<RonToken>) {
    return token.text.slice(1, -1);
}
