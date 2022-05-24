export type StringLiteral<T extends string> = string extends T ? never : T;

export interface EnumVariant<Literal extends string, Value> {
    name: StringLiteral<Literal>;
    value: Value;
}

export type TupleStruct<Literal extends string, Value extends any[]> = {
    name: StringLiteral<Literal>;
    value: Value;
};

export interface RonDecoder<V> {
    (string: string): V;
}

class TupleStruct2<V extends any[]> implements RonDecoder<V> {
    // readonly name: String;

    constructor(name: String) {
        // this.name = name;
    }

    parse(string: string): V {
        throw "unimplemented";
    }
}

export function tupleStruct<V extends any[]>(
    name: String,
    fields: FieldsArray<V>
): TupleStruct2<V> {
    return new TupleStruct2<V>(name);
}

type FieldsArray<V extends any[]> = {
    [Key in keyof V]: RonDecoder<V[Key]>;
};
