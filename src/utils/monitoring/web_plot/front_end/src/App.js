import './App.css';
import Graph from './components/Graph';
import SingleChart from './components/SingleChart';
import ResponsiveAppBar from './components/ResponsiveAppBar';
import { Route, Routes} from 'react-router-dom'

function App() {
  let socket = new WebSocket("ws://127.0.0.1:3012");
  return (
    <div className="App" style={{backgroundColor:"#211f1a"}}>
      <ResponsiveAppBar />
      <div className='content'>
        <Routes>
          <Route exact path="/" element={ <Graph socket={socket} /> } />
          <Route path="/Chart/:filename" element={ <SingleChart socket={socket} /> } />
        </Routes>
      </div>
    </div>
  );
}

export default App;
