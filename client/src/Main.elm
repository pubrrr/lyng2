port module Main exposing (Msg(..), main, update, view)

import Browser
import Html exposing (Html, div, text)
import Html.Attributes exposing (class, contenteditable)
import Html.Events exposing (on)
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
    = Something Value
    | Incoming String


type alias Model =
    String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Something value ->
            ( model, sendMessage value )

        Incoming string ->
            ( string, Cmd.none )


view : Model -> Html Msg
view model =
    div []
        [ div [ class "editorContainer" ]
            [ div
                [ contenteditable True
                , class "editorWindow"
                , on "input" (Json.Decode.value |> Json.Decode.map (\value -> Something value))
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
