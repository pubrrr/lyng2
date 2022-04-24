port module Main exposing (Msg(..), main, update, view)

import Browser
import Html exposing (Html, div, option, select, text)
import Html.Attributes exposing (class, contenteditable, value)
import Html.Events exposing (on, onInput)
import Interface exposing (EvaluationResult(..), parseEvaluationResult)
import Json.Decode
import Json.Encode exposing (Value, string)


port sendMessage : Value -> Cmd msg


port messageReceiver : (String -> msg) -> Sub msg


main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type Msg
    = Outgoing Value
    | Incoming String
    | ChangeLanguage String


type alias Model =
    String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Outgoing value ->
            ( model, sendMessage value )

        Incoming string ->
            case parseEvaluationResult string of
                Ok (Success result) ->
                    ( result, Cmd.none )

                Ok (Error error) ->
                    ( error, Cmd.none )

                Err error ->
                    ( error, Cmd.none )

        ChangeLanguage string ->
            ( model ++ string, Cmd.none )


view : Model -> Html Msg
view model =
    div []
        [ select [ onInput ChangeLanguage ]
            [ option [ value "lyng2-Math" ] [ text "lyng2 - Maths edition" ]
            , option [ value "other" ] [ text "whatever other fancy language" ]
            ]
        , div [ class "editorContainer" ]
            [ div
                [ contenteditable True
                , class "editorWindow"
                , on "input" (Json.Decode.value |> Json.Decode.map (\value -> Outgoing value))
                ]
                [ text "" ]
            , div [ class "editorWindow" ] [ text model ]
            ]
        ]


init : () -> ( Model, Cmd Msg )
init _ =
    ( "result...", Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ =
    messageReceiver Incoming
