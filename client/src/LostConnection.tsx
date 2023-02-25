import { getSubscribers } from 'react-use-websocket/dist/lib/manage-subscribers';
import { websocketUrl } from './Editor';
import { Navigate } from 'react-router-dom';

const LostConnection = () => {
    const reconnectWebsocket = () => {
        getSubscribers(websocketUrl).forEach((subscriber) => subscriber.reconnect.current());
        return <Navigate to={'/'} />;
    };

    return (
        <div className='connection-lost'>
            Lost Connection
            <button onClick={reconnectWebsocket}>Retry</button>
        </div>
    );
};

export default LostConnection;
