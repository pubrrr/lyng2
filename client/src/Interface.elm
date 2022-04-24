module Interface exposing (EvaluationResult(..), parseEvaluationResult)

import Parser exposing ((|.), (|=), DeadEnd, Parser, oneOf, run, succeed, symbol, variable)
import Set


type EvaluationResult
    = Success String
    | Error String


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
