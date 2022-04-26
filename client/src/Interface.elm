module Interface exposing (EvaluationResult(..), parseEvaluationResult)

import Ron exposing (Value(..), fromString, variantFunction, withField)


type EvaluationResult
    = Success String
    | Error String


ronEvaluationResult : Value EvaluationResult
ronEvaluationResult =
    Enum [ variantFunction Success "Success" |> withField, variantFunction Error "Error" |> withField ]


parseEvaluationResult : String -> Result String EvaluationResult
parseEvaluationResult =
    fromString ronEvaluationResult
