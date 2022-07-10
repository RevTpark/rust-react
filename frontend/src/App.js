
import { BrowserRouter as Router,
   Switch, 
  Route,
   Link,
   Routes} from "react-router-dom";
import './App.css';

import Login from './login';
import MultipleInputs from './MultipleInputs';
function App() {
  return (
    <div className='container'>
     <Router>
      
    

      
      <Routes>
        <Route path="Login/*" element={<Login/>} />
        <Route path="MultipleInputs/*" element={<MultipleInputs />} />
      </Routes>
     
     </Router>
    </div>
   

  );
}

export default App;