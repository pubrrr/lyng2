module RonTest exposing (suite)

import Expect exposing (equal)
import Parser exposing (int)
import Ron exposing (Value(..), bool, fromString, string, variant, withField)
import Test exposing (..)


suite : Test
suite =
    describe "Suite"
        [ describe "Decoding" decodeTests
        ]


decodeTests =
    [ test "variant with 0 fields"
        (\_ -> fromString testEnumValue "Variant0" |> equal (Ok VariantWith0Fields))
    , test "variant with 1 field"
        (\_ -> fromString testEnumValue "Variant1( \"value\")" |> equal (Ok (VariantWith1Field "value")))
    , test
        "variant with 2 fields"
        (\_ ->
            fromString testEnumValue "Variant2(\"value1\", \"value2\" ) "
                |> equal (Ok (VariantWith2Fields "value1" "value2"))
        )
    , test "variant with 3 fields"
        (\_ ->
            fromString testEnumValue "Variant3(true, \"value2\" , \"value3\",)"
                |> equal (Ok (VariantWith3Fields True "value2" "value3"))
        )
    , test "variant with 5 fields"
        (\_ ->
            fromString testEnumValue "Variant5(\"value1\" , \"value2\", 5, \"value4\", \"value5\")"
                |> equal (Ok (VariantWith5Fields "value1" "value2" 5 "value4" "value5"))
        )
    ]


type TestEnum
    = VariantWith0Fields
    | VariantWith1Field String
    | VariantWith2Fields String String
    | VariantWith3Fields Bool String String
    | VariantWith5Fields String String Int String String


testEnumValue : Value TestEnum
testEnumValue =
    Enum
        [ variant VariantWith5Fields "Variant5" |> withField string |> withField string |> withField int |> withField string |> withField string
        , variant VariantWith3Fields "Variant3" |> withField bool |> withField string |> withField string
        , variant VariantWith2Fields "Variant2" |> withField string |> withField string
        , variant VariantWith1Field "Variant1" |> withField string
        , variant VariantWith0Fields "Variant0"
        ]
