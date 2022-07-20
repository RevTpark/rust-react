import React, { useState, useEffect } from 'react'
import { Link } from 'react-router-dom';
import './App.css'

const MultipleInputs = () => {
  const initialValues={Username:"",email:"",Phone:"",Password:""};

  const [userregistration, setUserregisteration] = useState(initialValues);
  const [Records, setRecords] = useState([])
  const [formErrors, setformErrors] = useState({})
  const [isSubmit, setisSubmit] = useState(false);
  const handleInput = (e)=>{
const{ name,value }=e.target;
setUserregisteration({...userregistration,[name]:value});
console.log(userregistration);
  }

  const handleSubmit = (e)=>{
e.preventDefault();
const newRegistration={...userregistration, id:new Date().getTime().toString()}
console.log(Records);
setRecords([...Records,newRegistration])
setformErrors(validate(userregistration));
setisSubmit(true);
  }
  useEffect(()=>{
console.log(formErrors);
    if(Object.keys(formErrors).length === 0 &&  isSubmit){
      console.log(userregistration);
    }
  },[formErrors]);
  const validate=(values)=>{
const errors={};
const regex=/^[a-zA-Z0-9.! #$%&'*+/=? ^_`{|}~-]+@[a-zA-Z0-9-]+(?:\. [a-zA-Z0-9-]+)*$/;
// const passwordRegex="^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*#?&])[A-Za-z\d@$!%*#?&]{8,}$";
if(!values.Username){
  errors.Username="Required Field!"
}
if(!values.email){
  errors.email="Required Field!"
} else if(!regex.test(values.email)){
  errors.email="Not a valid email format!"
}
if(!values.Phone){
  errors.Phone="Required Field!"
}
if(!values.Password){
  errors.Password="Required Field!"
} 
else if(values.Password.length < 4){
  errors.Password="Invalid Password"
}else if(values.Password.length>10){
  errors.Password="Invalidddddd Password"
}
return errors;
  }
  return (
    <div className='pagess'>
    <div className='login'>
        <div class="header">
    <h1>SIGN UP</h1>
  </div>
    <form action="" className='form' onSubmit={handleSubmit}>
      <div className='form-controll'>
        <input type="text" autoComplete ="off"
        value={userregistration.Username}
        onChange={handleInput}
        name='Username' id='Username' className='input'  placeholder='Username'/>
         
      </div>
      <p className = "para">{formErrors.Username}</p>
      <div className='form-controll' >
        {/* <label className='label'>Email</label> */}
        <input type="text" autoComplete ="off"
        value={userregistration.email}
        onChange={handleInput}
        name='email' id='email' className='input' placeholder='Email'/>
      </div>
      <p className = "para">{formErrors.email}</p>
      <div className='form-controll'>
        {/* <label className='label'>Phone</label> */}
        <input type="tel" autoComplete ="off"
        value={userregistration.Phone}
        onChange={handleInput}
        name='Phone' id='Phone' className='input' placeholder='Phone' />
      </div>
      <p className = "para">{formErrors.Phone}</p>
      <div className='form-controll'>
        {/* <label className='label'>Password</label> */}
        <input type="password" autoComplete ="off"
        value={userregistration.Password}
        onChange={handleInput}
        name='Password' id='Password' className='input' placeholder='Password' />
      </div>
      <p className = "para">{formErrors.Password}</p>
      <button type='submit' className='buttonn' >REGISTER</button>
      <div className='Switch'>
      <Link to='/Login' >LOGIN</Link>
      </div>
    </form>
    </div>
    </div>
   
  )
}

export default MultipleInputs