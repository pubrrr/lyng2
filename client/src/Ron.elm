module Ron exposing (Value(..), bool, fromString, string, variant, withField)

import Parser exposing ((|.), (|=), DeadEnd, Parser, oneOf, run, spaces, succeed, symbol, variable)
import Set


type Value value
    = Enum (List (Variant value))


fromString : Value value -> String -> Result String value
fromString value input =
    case value of
        Enum variants ->
            input
                |> run (oneOf (List.map variantParser variants))
                |> Result.mapError deadEndsToString


type Variant value
    = WithoutFields value String
    | WithFields (Parser value)


variant : constructor -> String -> Variant constructor
variant value name =
    WithoutFields value name


withField : Parser field -> Variant (field -> partialConstructor) -> Variant partialConstructor
withField fieldParser variant_ =
    case variant_ of
        WithoutFields value name ->
            WithFields <|
                succeed value
                    |. symbol name
                    |. symbol "("
                    |= fieldParser

        WithFields withFields ->
            WithFields <|
                withFields
                    |. symbol ","
                    |. spaces
                    |= fieldParser


variantParser : Variant value -> Parser value
variantParser variant_ =
    case variant_ of
        WithoutFields constructor name ->
            succeed constructor
                |. symbol name

        WithFields variantWithFields ->
            variantWithFields
                |. oneOf [ symbol ",", succeed () ]
                |. spaces
                |. symbol ")"


bool : Parser Bool
bool =
    oneOf
        [ succeed True |. symbol "true"
        , succeed False |. symbol "false"
        ]


string : Parser String
string =
    succeed identity
        |. spaces
        |. symbol stringDelimiter
        |= oneOf
            [ variable
                { start = \c -> String.fromChar c /= stringDelimiter
                , inner = \c -> String.fromChar c /= stringDelimiter
                , reserved = Set.empty
                }
            , succeed ""
            ]
        |. symbol stringDelimiter
        |. spaces


stringDelimiter : String
stringDelimiter =
    "\""


deadEndsToString : List DeadEnd -> String
deadEndsToString deadEnds =
    deadEnds
        |> List.map (\deadEnd -> "Parsing Problem: " ++ Debug.toString deadEnd)
        |> String.join ", "
