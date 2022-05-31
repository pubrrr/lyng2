import { ParseResult, ParserOutput, Token } from "typescript-parsec";
import { RonToken } from "./common";
import { string } from "./parser";

export function number(token: Token<RonToken> | undefined): ParserOutput<RonToken, number> {
    const stringResult = string(token);

    if (!stringResult.successful) {
        return stringResult;
    }

    const parsedNumberCandidates = stringResult.candidates.map(tryToParseToNumber);

    let unparseableString = parsedNumberCandidates.find((value) => typeof value.result == "string");
    if (unparseableString != undefined) {
        return createErrorFor(unparseableString as ParseResult<RonToken, string>);
    }

    return {
        ...stringResult,
        candidates: parsedNumberCandidates as ParseResult<RonToken, number>[],
    };
}

function tryToParseToNumber(
    candidate: ParseResult<RonToken, string>
): ParseResult<RonToken, string> | ParseResult<RonToken, number> {
    if (!looksLikeANumber(candidate)) {
        return candidate;
    }

    const number = parseFloat(candidate.result);
    if (!isNaN(number)) {
        return {
            ...candidate,
            result: number,
        };
    }

    return candidate;
}

function looksLikeANumber(value: ParseResult<RonToken, string>) {
    return /^\d+$/.test(value.result) || /^\d*\.\d*$/.test(value.result);
}

function createErrorFor(
    unparsableString: ParseResult<RonToken, string>
): ParserOutput<RonToken, number> {
    return {
        successful: false,
        error: {
            kind: "Error",
            pos: undefined,
            message: "could not parse " + unparsableString.result + " to a number",
        },
    };
}
