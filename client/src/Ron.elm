module Ron exposing (Value(..), Variant, fromString, variant)

import Parser exposing ((|.), (|=), DeadEnd, Parser, oneOf, run, succeed, symbol, variable)
import Set


type Value a
    = Enum (List (Variant a))


fromString : Value a -> String -> Result String a
fromString value input =
    case value of
        Enum variants ->
            input
                |> run (oneOf (List.map variantParser variants))
                |> Result.mapError deadEndsToString


type alias Variant a =
    { name : String
    , constructor : String -> a
    }


variant : (String -> a) -> String -> Variant a
variant constructor name =
    { name = name, constructor = constructor }


variantParser : Variant a -> Parser a
variantParser theVariant =
    succeed theVariant.constructor
        |. symbol theVariant.name
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
