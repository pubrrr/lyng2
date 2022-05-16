import React, {useEffect, useState} from 'react';
import Loading from "./Loading";
import LoadingFailed from "./LoadingFailed";
import Editor from "./Editor";
import LostConnection from "./LostConnection";

const enum State {
    Initializing,
    Initialized,
    Failed,
    LostConnection,
}

function App(): JSX.Element {
    let [state, setState] = useState(State.Initializing);

    useEffect(() => {
        const socket = new WebSocket('ws://127.0.0.1:8080');

        socket.onopen = () => setState(State.Initialized);
        socket.onerror = () => setState(State.Failed);
        socket.onclose = () => setState(State.LostConnection);
    }, []);

    switch (state) {
        case State.Initializing:
            return <Loading/>
        case State.Failed:
            return <LoadingFailed/>
        case State.Initialized:
            return <Editor/>
        case State.LostConnection:
            return <LostConnection/>
    }
}

export default App;
