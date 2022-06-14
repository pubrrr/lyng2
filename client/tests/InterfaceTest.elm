module InterfaceTest exposing (suite)

import Expect exposing (equal, err)
import Fuzz exposing (string)
import Interface exposing (EvaluationResult(..), parseEvaluationResult)
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
                parseEvaluationResult ("Success(\"" ++ messageWithoutQuotes ++ "\")")
                    |> equal (Ok (Success messageWithoutQuotes))
            )
        , fuzz string
            "valid error"
            (\message ->
                let
                    messageWithoutQuotes =
                        String.replace "\"" "" message
                in
                parseEvaluationResult ("Error(\"" ++ messageWithoutQuotes ++ "\")")
                    |> equal (Ok (Error messageWithoutQuotes))
            )
        , test "parsing error because of misplaced quotes"
            (\_ -> parseEvaluationResult "Success(\"\"\")" |> err)
        , test "parsing error"
            (\_ -> parseEvaluationResult "unknown glibberish" |> err)
        ]
