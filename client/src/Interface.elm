module Interface exposing (EvaluationResult(..), decodeEvaluationResult, parseEvaluationResult)

import Json.Decode exposing (Decoder, andThen, string)
import Parser exposing ((|.), (|=), DeadEnd, Parser, oneOf, run, succeed, symbol, variable)
import Set


type EvaluationResult
    = Success String
    | Error String


decodeEvaluationResult : Decoder EvaluationResult
decodeEvaluationResult =
    Json.Decode.string
        |> andThen
            (\string ->
                let
                    string1 =
                        string

                    c =
                        Debug.log "c " '"'
                in
                case parseEvaluationResult (Debug.log "" string1) of
                    Ok success ->
                        Json.Decode.succeed success

                    Err deadEnds ->
                        Json.Decode.fail deadEnds
            )


parseEvaluationResult : String -> Result String EvaluationResult
parseEvaluationResult string =
    string
        |> run (oneOf [ successParser, errorParser ])
        |> Result.mapError deadEndsToString


successParser : Parser EvaluationResult
successParser =
    stringVariantParser Success "Success"


errorParser : Parser EvaluationResult
errorParser =
    stringVariantParser Error "Error"


stringVariantParser : (String -> EvaluationResult) -> String -> Parser EvaluationResult
stringVariantParser variantConstructor variantName =
    succeed variantConstructor
        |. symbol variantName
        |. symbol "("
        |. symbol "\""
        |= oneOf
            [ variable
                { start = \c -> c /= '"'
                , inner = \c -> c /= '"'
                , reserved = Set.empty
                }
            , succeed ""
            ]
        |. symbol "\""
        |. symbol ")"


deadEndsToString : List DeadEnd -> String
deadEndsToString deadEnds =
    deadEnds
        |> List.map (\deadEnd -> "Parsing Problem: " ++ Debug.toString deadEnd)
        |> String.join ", "
