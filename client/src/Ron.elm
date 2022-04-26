module Ron exposing (Value(..), fromString, toString, variantFunction, withField)

import Parser exposing ((|.), (|=), DeadEnd, Parser, Problem(..), oneOf, run, spaces, succeed, symbol, variable)
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
    = WithoutFields a String
    | WithFields (VariantWithFields a)


type VariantWithFields a
    = OuterVariantDecorator (String -> a) String
    | InnerVariantDecorator (VariantWithFields (String -> a))


variantFunction : a -> String -> Variant a
variantFunction value name =
    WithoutFields value name


withField : Variant (String -> b) -> Variant b
withField variant =
    case variant of
        WithoutFields a string ->
            WithFields (OuterVariantDecorator a string)

        WithFields withFields ->
            WithFields (InnerVariantDecorator withFields)


variantParser : Variant a -> Parser a
variantParser variant =
    case variant of
        WithoutFields constructor name ->
            succeed constructor
                |. symbol name

        WithFields variantWithFields ->
            variantParserWithFieldsParser variantWithFields


variantParserWithFieldsParser : VariantWithFields a -> Parser a
variantParserWithFieldsParser variantWithFields =
    case variantWithFields of
        OuterVariantDecorator constructor name ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser
                |. symbol ")"

        InnerVariantDecorator wrapped ->
            flip wrapped
                |. symbol ","
                |= stringParser
                |. oneOf [ symbol ",", succeed () ]
                |. spaces
                |. symbol ")"


flip : VariantWithFields (String -> a) -> Parser (String -> a)
flip variant =
    case variant of
        OuterVariantDecorator constructor name ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser

        InnerVariantDecorator wrapped ->
            flop wrapped
                |. symbol ","
                |. spaces
                |= stringParser


flop : VariantWithFields (String -> a) -> Parser (String -> a)
flop variant =
    case variant of
        OuterVariantDecorator constructor name ->
            succeed constructor
                |. symbol name
                |. symbol "("
                |= stringParser

        InnerVariantDecorator wrapped ->
            flip wrapped
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
