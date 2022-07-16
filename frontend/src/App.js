
import { BrowserRouter as Router,
   Switch, 
  Route,
   Link,
   Routes} from "react-router-dom";
import  Navbar from './components/Mainpage/Navbar/Navbar'
import Login from './components/login page/login';
import MultipleInputs from './components/login page/MultipleInputs';
import HomePage from './components/pages/HomePage'
import Products from "./components/pages/Products";
import 'bootstrap/dist/css/bootstrap.min.css';  
function App() {
  return (
    <>
    <div className='containerr'>
     <Router>  
      <Navbar/>
      <Routes>
       <Route path="/" element={<HomePage/>} />
        <Route path="Login/*" element={<Login/>} />
        <Route path="MultipleInputs/*" element={<MultipleInputs />} />
        <Route path="Products/*" element={<Products/>} />
        
      </Routes>
     
     </Router>
    </div>
   
    </>

  );
}

export default App;