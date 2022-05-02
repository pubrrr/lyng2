module Interface exposing (EvaluationResult(..), parseEvaluationResult)

import Ron exposing (Value(..), fromString, string, variant, withField)


type EvaluationResult
    = Success String
    | Error String


ronEvaluationResult : Value EvaluationResult
ronEvaluationResult =
    Enum [ variant Success "Success" |> withField string, variant Error "Error" |> withField string ]


parseEvaluationResult : String -> Result String EvaluationResult
parseEvaluationResult =
    fromString ronEvaluationResult
