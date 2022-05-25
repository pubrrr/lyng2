import { decode, string, tupleStruct, TupleStruct } from "./parser";

describe("decoding RON strings", () => {
    test("should succeed on a valid string", () => {
        let result = decode('"valid string"', string);

        expect(result.success).toBe(true);
        expect((result as { value: string }).value).toBe("valid string");
    });

    test("should succeed on a valid string without spaces", () => {
        let result = decode('"a"', string);

        expect(result.success).toBe(true);
        expect((result as { value: string }).value).toBe("a");
    });

    test("should fail with missing quotes", () => {
        let result = decode("has no quotes", string);

        expect(result.success).toBe(false);
    });

    test("should fail with missing closing double quotes", () => {
        let result = decode('"has no closing quotes', string);

        expect(result.success).toBe(false);
    });

    test("should fail on double quotes in the middle", () => {
        let result = decode('"misplaced " quote"', string);

        expect(result.success).toBe(false);
    });
});

describe("decoding RON tuple structs", () => {
    test("should decode a valid tuple struct with one string field", () => {
        let underTest = tupleStruct("Success", [string]);

        let result = decode('Success("success message")', underTest);

        expect(result.success).toBe(true);
        expect(
            (result as { value: TupleStruct<"Success", string[]> }).value
        ).toEqual({
            name: "Success",
            value: ["success message"],
        });
    });

    test("should decode a tuple struct with trailing commas", () => {
        let underTest = tupleStruct("Success", [string]);

        let result = decode('Success("success message", )', underTest);

        expect(result.success).toBe(true);
        expect(
            (result as { value: TupleStruct<"Success", string[]> }).value
        ).toEqual({
            name: "Success",
            value: ["success message"],
        });
    });

    test("should decode a valid tuple struct with two string fields", () => {
        let underTest = tupleStruct("Success", [string, string]);

        let result = decode('Success("first","second")', underTest);

        expect(result.success).toBe(true);
        expect(
            (result as { value: TupleStruct<"Success", string[]> }).value
        ).toEqual({
            name: "Success",
            value: ["first", "second"],
        });
    });

    test("should decode a tuple struct with spaces between fields", () => {
        let underTest = tupleStruct("Success", [string, string]);

        let result = decode('Success("first" , "second")', underTest);

        expect(result.success).toBe(true);
        expect(
            (result as { value: TupleStruct<"Success", string[]> }).value
        ).toEqual({
            name: "Success",
            value: ["first", "second"],
        });
    });
});
