port module Main exposing (Msg(..), main)

import Browser
import Html exposing (Html, button, div, option, select, text)
import Html.Attributes exposing (class, contenteditable, value)
import Html.Events exposing (on, onClick, onInput)
import Interface exposing (EvaluationResult(..), parseEvaluationResult)
import Json.Decode as JD
import Ron exposing (Value(..), fromString, variant)


port sendMessage : String -> Cmd msg


port messageReceiver : (String -> msg) -> Sub msg


main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type Msg
    = SetEditorContent String
    | SetViewContent String
    | ChangeLanguage String
    | SendEditorContent


type alias Model =
    { editorContent : String
    , viewContent : String
    , language : Language
    }


type Language
    = Lyng2Math
    | Other


languageParser : Value Language
languageParser =
    Enum
        [ variant Lyng2Math lyng2MathsEdition
        , variant Other otherLanguage
        ]


lyng2MathsEdition =
    "Lyng2MathsEdition"


{-| TODO clean this up and replace by a proper language
-}
otherLanguage =
    "other"


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetEditorContent value ->
            ( { model | editorContent = value }, Cmd.none )

        SetViewContent string ->
            ( { model | viewContent = updateViewContent string }, Cmd.none )

        ChangeLanguage language ->
            ( model |> updateLanguage language, Cmd.none )

        SendEditorContent ->
            ( model, sendMessage model.editorContent )


updateViewContent : String -> String
updateViewContent string =
    case parseEvaluationResult string of
        Ok (Success result) ->
            result

        Ok (Error error) ->
            error

        Err error ->
            error


updateLanguage : String -> Model -> Model
updateLanguage string model =
    case fromString languageParser string of
        Ok language ->
            { model | language = language }

        Err error ->
            { model | viewContent = "Selected unknown language \"" ++ string ++ "\": " ++ error }


view : Model -> Html Msg
view model =
    div []
        [ select [ onInput ChangeLanguage ]
            [ option [ value lyng2MathsEdition ] [ text "lyng2 - Maths edition" ]
            , option [ value otherLanguage ] [ text "whatever other fancy language" ]
            ]
        , button [ onClick SendEditorContent ] [ text "send" ]
        , div [ class "editorContainer" ]
            [ div
                [ contenteditable True
                , class "editorWindow"
                , on "input" (JD.at [ "target", "innerText" ] JD.string |> JD.map SetEditorContent)
                ]
                [ text "" ]
            , div [ class "editorWindow" ] [ text model.viewContent ]
            ]
        ]


init : () -> ( Model, Cmd Msg )
init _ =
    ( { editorContent = "", viewContent = "result...", language = Lyng2Math }, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ =
    messageReceiver SetViewContent
