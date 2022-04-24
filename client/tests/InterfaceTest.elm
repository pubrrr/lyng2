module InterfaceTest exposing (suite)

import Expect exposing (Expectation, equal, err)
import Fuzz exposing (string)
import Interface exposing (EvaluationResult(..), decodeEvaluationResult)
import Json.Decode exposing (decodeValue)
import Json.Encode
import Test exposing (..)


suite : Test
suite =
    describe "EvaluationResult decoding"
        [ fuzz string
            "success"
            (\message ->
                let
                    messageWithoutQuotes =
                        String.replace "\"" "" message
                in
                apply decodeEvaluationResult ("Success(\"" ++ messageWithoutQuotes ++ "\")")
                    |> equal (Ok (Success messageWithoutQuotes))
            )
        , fuzz string
            "valid error"
            (\message ->
                let
                    messageWithoutQuotes =
                        String.replace "\"" "" message
                in
                apply decodeEvaluationResult ("Error(\"" ++ messageWithoutQuotes ++ "\")")
                    |> equal (Ok (Error messageWithoutQuotes))
            )
        , test "parsing error because of misplaced quotes"
            (\_ -> apply decodeEvaluationResult "Success(\"\"\")" |> err)
        , test "parsing error"
            (\_ -> apply decodeEvaluationResult "unknown glibberish" |> err)
        ]


apply decoder string =
    decodeValue decoder (Json.Encode.string string)
