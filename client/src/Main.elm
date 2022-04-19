port module Main exposing (Msg(..), main, update, view)

import Browser
import Html exposing (Html, button, div, text)
import Html.Attributes exposing (class, contenteditable)
import Html.Events exposing (on, onClick)
import Json.Decode
import Json.Encode exposing (Value, string)


port sendMessage : Value -> Cmd msg


main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type Msg
    = Increment
    | Decrement
    | Something Value


type alias Model =
    Int


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Increment ->
            ( model + 1, sendMessage (string "increment") )

        Decrement ->
            ( model - 1, sendMessage (string "decrement") )

        Something value ->
            ( model - 1, sendMessage value )


view : Model -> Html Msg
view model =
    div []
        [ button [ onClick Decrement ] [ text "-" ]
        , div [] [ text (String.fromInt model) ]
        , button [ onClick Increment ] [ text "+" ]
        , div [ class "editorContainer" ]
            [ div [ contenteditable True, class "editorWindow", on "input" (Json.Decode.value |> Json.Decode.map (\value -> Something value)) ] [ text "" ]
            , div [ class "editorWindow" ] [ text "result..." ]
            ]
        ]


init : () -> ( Model, Cmd Msg )
init _ =
    ( 0, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none
