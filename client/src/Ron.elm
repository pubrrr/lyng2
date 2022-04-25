module Ron exposing (Value(..), Variant(..), fromString, toString)

import Parser exposing ((|.), (|=), DeadEnd, Parser, oneOf, run, spaces, succeed, symbol, variable)
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


type Variant a
    = Variant0 String a
    | Variant1 String (String -> a)
    | Variant2 String (String -> String -> a)


variantParser : Variant a -> Parser a
variantParser variant =
    case variant of
        Variant0 name constructor ->
            succeed constructor
                |. symbol name

        Variant1 name constructor ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser
                |. symbol ")"

        Variant2 name constructor ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser
                |. symbol ","
                |. spaces
                |= stringParser
                |. symbol ")"


stringParser : Parser String
stringParser =
    succeed identity
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


deadEndsToString : List DeadEnd -> String
deadEndsToString deadEnds =
    deadEnds
        |> List.map (\deadEnd -> "Parsing Problem: " ++ Debug.toString deadEnd)
        |> String.join ", "


toString : Value a -> a -> String
toString valueDefinition value =
    case valueDefinition of
        Enum variants ->
            variants |> List.map (printOrEmpty value) |> String.concat


printOrEmpty : a -> Variant a -> String
printOrEmpty value variant =
    case variant of
        Variant0 name constructor ->
            if value == constructor then
                name

            else
                ""

        Variant1 name constructor ->
            ""

        Variant2 name constructor ->
            ""
