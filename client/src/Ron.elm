module Ron exposing (Value(..), fromString, toString, variantFunction, withField)

import Parser exposing ((|.), (|=), DeadEnd, Parser, Problem(..), oneOf, run, spaces, succeed, symbol, variable)
import Set


type Value a
    = Enum (List (VariantWrapper a))


fromString : Value a -> String -> Result String a
fromString value input =
    case value of
        Enum variants ->
            input
                |> run (oneOf (List.map variantParser variants))
                |> Result.mapError deadEndsToString


type VariantWrapper a
    = VariantWrapperLeaf a String
    | OuterVariantDecorator (String -> a) String
    | InnerVariantDecorator (VariantWrapper (String -> a))


variantFunction : a -> String -> VariantWrapper a
variantFunction value name =
    VariantWrapperLeaf value name


withField : VariantWrapper (String -> b) -> VariantWrapper b
withField variant =
    case variant of
        VariantWrapperLeaf a string ->
            OuterVariantDecorator a string

        _ ->
            InnerVariantDecorator variant


variantParser : VariantWrapper a -> Parser a
variantParser variant =
    case variant of
        VariantWrapperLeaf constructor name ->
            succeed constructor
                |. symbol name

        OuterVariantDecorator constructor name ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser
                |. symbol ")"

        InnerVariantDecorator wrapped ->
            variantParser2 wrapped
                |. symbol ","
                |= stringParser
                |. oneOf [ symbol ",", succeed () ]
                |. spaces
                |. symbol ")"


variantParser2 : VariantWrapper (String -> a) -> Parser (String -> a)
variantParser2 variant =
    case variant of
        VariantWrapperLeaf constructor name ->
            succeed constructor
                |. symbol name

        OuterVariantDecorator constructor name ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser

        InnerVariantDecorator wrapped ->
            variantParser3 wrapped
                |. symbol ","
                |. spaces
                |= stringParser


variantParser3 : VariantWrapper (String -> a) -> Parser (String -> a)
variantParser3 variant =
    case variant of
        VariantWrapperLeaf constructor name ->
            succeed constructor
                |. symbol name

        OuterVariantDecorator constructor name ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser

        InnerVariantDecorator wrapped ->
            variantParser2 wrapped
                |. symbol ","
                |. spaces
                |= stringParser


stringParser : Parser String
stringParser =
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


toString : Value a -> a -> String
toString valueDefinition value =
    "todo"



--case valueDefinition of
--    Enum variants ->
--        variants |> List.map (printOrEmpty value) |> String.concat
--printOrEmpty : a -> VariantWrapper a -> String
--printOrEmpty value variant =
--    case variant of
--        VariantWrapperLeaf constructor string ->
--            if value == constructor then
--                string
--
--            else
--                ""
--
--        VariantDecorator wrapped ->
--            printOrEmpty2 (\_ -> value) wrapped
--
--
--printOrEmpty2 : (String -> a) -> VariantWrapper (String -> a) -> String
--printOrEmpty2 value variant =
--    case variant of
--        VariantWrapperLeaf constructor string ->
--            if value == constructor then
--                string
--
--            else
--                ""
--
--        VariantDecorator wrapped ->
--            printOrEmpty (\_ -> value) wrapped
