module RonTest exposing (suite)

import Expect exposing (Expectation, equal)
import Ron exposing (Value(..), fromString, toString, variantFunction, withField)
import Test exposing (..)


suite : Test
suite =
    describe "Suite"
        [ describe "Decoding" decodeTests
        , describe "Encoding" encodeTests
        ]


decodeTests =
    [ test "variant with 0 fields"
        (\_ -> fromString testEnumValue "Variant0" |> equal (Ok VariantWith0Fields))
    , test "variant with 1 field"
        (\_ -> fromString testEnumValue "Variant1( \"value\")" |> equal (Ok (VariantWith1Field "value")))
    , test
        "variant with 2 fields"
        (\_ -> fromString testEnumValue "Variant2(\"value1\", \"value2\" ) " |> equal (Ok (VariantWith2Fields "value1" "value2")))
    , test "variant with 3 fields"
        (\_ -> fromString testEnumValue "Variant3(\"value1\", \"value2\" , \"value3\",)" |> equal (Ok (VariantWith3Fields "value1" "value2" "value3")))
    , test "variant with 5 fields"
        (\_ ->
            fromString testEnumValue "Variant5(\"value1\" , \"value2\", \"value3\", \"value4\", \"value5\")"
                |> equal (Ok (VariantWith5Fields "value1" "value2" "value3" "value4" "value5"))
        )
    ]


encodeTests =
    [ test "dummy" (\_ -> 1 |> equal 1)

    --[ test "variant with 0 fields"
    --    (\_ -> toString testEnumValue VariantWith0Fields |> equal "Variant0")
    --, test "variant with 1 field"
    --    (\_ -> toString testEnumValue (VariantWith1Field "value") |> equal "Variant1(\"value\")")
    --, test "variant with 2 fields"
    --    (\_ -> toString testEnumValue (VariantWith2Fields "value1" "value2") |> equal "Variant2(\"value1\", \"value2\")")
    ]


type TestEnum
    = VariantWith0Fields
    | VariantWith1Field String
    | VariantWith2Fields String String
    | VariantWith3Fields String String String
    | VariantWith5Fields String String String String String


testEnumValue : Value TestEnum
testEnumValue =
    Enum
        [ variantFunction VariantWith5Fields "Variant5" |> withField |> withField |> withField |> withField |> withField
        , variantFunction VariantWith3Fields "Variant3" |> withField |> withField |> withField
        , variantFunction VariantWith2Fields "Variant2" |> withField |> withField
        , variantFunction VariantWith1Field "Variant1" |> withField
        , variantFunction VariantWith0Fields "Variant0"
        ]
