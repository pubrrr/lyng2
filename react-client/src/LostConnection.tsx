import { getSubscribers } from "react-use-websocket/dist/lib/manage-subscribers";
import { websocketUrl } from "./Editor";
import { useNavigate } from "react-router-dom";

const LostConnection = () => {
    const navigate = useNavigate();

    const reconnectWebsocket = () => {
        getSubscribers(websocketUrl).forEach((subscriber) => subscriber.reconnect.current());
        navigate("/");
    };

    return (
        <div className="connection-lost">
            Lost Connection
            <button onClick={reconnectWebsocket}>Retry</button>
        </div>
    );
};

export default LostConnection;
