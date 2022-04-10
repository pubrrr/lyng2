port module Main exposing (Msg(..), main, update, view)

import Browser
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)
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


type alias Model =
    Int


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Increment ->
            ( model + 1, sendMessage (string "increment") )

        Decrement ->
            ( model - 1, sendMessage (string "decrement") )


view : Model -> Html Msg
view model =
    div []
        [ button [ onClick Decrement ] [ text "-" ]
        , div [] [ text (String.fromInt model) ]
        , button [ onClick Increment ] [ text "+" ]
        ]


init : () -> ( Model, Cmd Msg )
init _ =
    ( 0, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none
