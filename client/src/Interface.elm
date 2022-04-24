module Interface exposing (EvaluationResult(..), parseEvaluationResult)

import Ron exposing (Value(..), fromString, variant)


type EvaluationResult
    = Success String
    | Error String


ronEvaluationResult : Value EvaluationResult
ronEvaluationResult =
    Enum [ variant Success "Success", variant Error "Error" ]


parseEvaluationResult : String -> Result String EvaluationResult
parseEvaluationResult =
    fromString ronEvaluationResult
