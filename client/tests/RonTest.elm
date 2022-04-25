module RonTest exposing (suite)

import Expect exposing (Expectation, equal)
import Ron exposing (Value(..), Variant(..), fromString, toString)
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
        (\_ -> fromString testEnumValue "Variant1(\"value\")" |> equal (Ok (VariantWith1Field "value")))
    , test "variant with 2 fields"
        (\_ -> fromString testEnumValue "Variant2(\"value1\", \"value2\")" |> equal (Ok (VariantWith2Fields "value1" "value2")))
    ]


encodeTests =
    [ test "variant with 0 fields"
        (\_ -> toString testEnumValue VariantWith0Fields |> equal "Variant0")
    , test "variant with 1 field"
        (\_ -> toString testEnumValue (VariantWith1Field "value") |> equal "Variant1(\"value\")")
    , test "variant with 2 fields"
        (\_ -> toString testEnumValue (VariantWith2Fields "value1" "value2") |> equal "Variant2(\"value1\", \"value2\")")
    ]


type TestEnum
    = VariantWith0Fields
    | VariantWith1Field String
    | VariantWith2Fields String String


testEnumValue : Value TestEnum
testEnumValue =
    Enum
        [ Variant0 "Variant0" VariantWith0Fields
        , Variant1 "Variant1" VariantWith1Field
        , Variant2 "Variant2" VariantWith2Fields
        ]
