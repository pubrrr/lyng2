import { decode, number, ronEnum, string, tupleStruct, TupleStruct } from "./index";

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

describe("decoding numbers", () => {
    test("should fail on an invalid number", () => {
        let result = decode('"12.34.56"', number);

        if (result.success) {
            throw result.value;
        }
        expect((result as { error: string }).error).toBe("could not parse 12.34.56 to a number");
    });

    test("should succeed on a valid integer", () => {
        let result = decode('"12"', number);

        if (!result.success) {
            throw result.error;
        }
        expect(result.value).toBe(12);
    });

    test("should succeed on a valid float", () => {
        let result = decode('"12.34"', number);

        if (!result.success) {
            throw result.error;
        }
        expect(result.value).toBeCloseTo(12.34, 3);
    });
});

describe("decoding RON tuple structs", () => {
    test("should decode a valid tuple struct with one string field", () => {
        let underTest = tupleStruct("Success", string);

        let result = decode('Success("success message")', underTest);

        expect(result.success).toBe(true);
        expect((result as { value: TupleStruct<"Success", string[]> }).value).toEqual({
            name: "Success",
            value: ["success message"],
        });
    });

    test("should decode a tuple struct with trailing commas", () => {
        let underTest = tupleStruct("Success", string);

        let result = decode('Success("success message", )', underTest);

        expect(result.success).toBe(true);
        expect((result as { value: TupleStruct<"Success", string[]> }).value).toEqual({
            name: "Success",
            value: ["success message"],
        });
    });

    test("should decode a valid tuple struct with two string fields", () => {
        let underTest = tupleStruct("Success", string, string);

        let result = decode('Success("first","second")', underTest);

        expect(result.success).toBe(true);
        expect((result as { value: TupleStruct<"Success", string[]> }).value).toEqual({
            name: "Success",
            value: ["first", "second"],
        });
    });

    test("should decode a tuple struct with spaces between fields", () => {
        let underTest = tupleStruct("Success", string, string);

        let result = decode('Success("first" , "second")', underTest);

        expect(result.success).toBe(true);
        expect((result as { value: TupleStruct<"Success", string[]> }).value).toEqual({
            name: "Success",
            value: ["first", "second"],
        });
    });

    test("should decode a tuple struct of string and int", () => {
        let underTest = tupleStruct("Success", string, number);

        let result = decode('Success("first" , "42")', underTest);

        expect(result.success).toBe(true);
        expect((result as { value: TupleStruct<"Success", [string, number]> }).value).toEqual({
            name: "Success",
            value: ["first", 42],
        });
    });

    test("should decode a string tuple struct that contains ()", () => {
        let underTest = tupleStruct("Success", string);

        let result = decode('Success("(with parenthesis)")', underTest);

        expect(result.success).toBe(true);
        expect((result as { value: TupleStruct<"Success", [string]> }).value).toEqual({
            name: "Success",
            value: ["(with parenthesis)"],
        });
    });
});

describe("decoding RON enums", () => {
    let name: "First" = "First";
    const underTest = ronEnum(tupleStruct(name, string), tupleStruct("Second", number, string));

    test("should decode first enum variant", () => {
        let result = decode('First("aValue")', underTest);

        if (!result.success) {
            throw result.error;
        }
        expect(result.value.name).toBe("First");
        expect(result.value.value).toStrictEqual(["aValue"]);
    });

    test("should decode second enum variant", () => {
        let result = decode('Second("42", "aValue")', underTest);

        if (!result.success) {
            throw result.error;
        }
        expect(result.value.name).toBe("Second");
        expect(result.value.value).toStrictEqual([42, "aValue"]);
    });

    test("should return an error when providing field for another variant", () => {
        let result = decode('Second("aValue")', underTest);

        if (result.success) {
            throw result.value;
        }
        expect(result.error).toBe('Did not find matching enum variant for "Second("aValue")"');
    });

    test("should return an error on invalid input", () => {
        let result = decode("something invalid", underTest);

        if (result.success) {
            throw result.value;
        }
        expect(result.error).toBe('Did not find matching enum variant for "something invalid"');
    });
});
