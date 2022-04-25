module Interface exposing (EvaluationResult(..), parseEvaluationResult)

import Ron exposing (Value(..), Variant(..), fromString)


type EvaluationResult
    = Success String
    | Error String


ronEvaluationResult : Value EvaluationResult
ronEvaluationResult =
    Enum [ Variant1 "Success" Success, Variant1 "Error" Error ]


parseEvaluationResult : String -> Result String EvaluationResult
parseEvaluationResult =
    fromString ronEvaluationResult
