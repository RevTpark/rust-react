import React from 'react'
import './Footer.css'
import {Link} from 'react-router-dom'
import { FaRegEnvelope ,FaArrowRight,FaFacebook,FaInstagram,FaTwitter,FaPinterestP} from "react-icons/fa";

function Footer() {
  return (
    <div className='footer-container'>
         <footer>
         <div className="row">
        <div className="col">
         <h3>Welcome</h3>
         <p>Subscribe to get the updates and offers on our latest products</p>
        </div> 
        <div className="col">
          <h3>LOCATION</h3>
          <p> Whitefield road <br/> Main Street </p>
          <p className='email-id'> abc@gmail.com</p>
          <h4>+91 - 987654321 </h4>

          </div> 
        <div className="col">
      <h3> Links</h3>
      <ul>
        <li>
        <Link className='Link' to='/' >Home</Link>
        </li>
        <li>
        <Link className='Link' to='/Products' >Products</Link>
        </li>
        <li>
        <Link className='Link' to='/Login' >Login</Link>
        </li>
        <li>
        <Link className='Link' to='/MultipleInputs' >Register</Link>
        </li>
      </ul>

          </div> 
        <div className="col">
        <h3>Contact us</h3>
            <form >
            <FaRegEnvelope className=' FaRegEnvelope'> </FaRegEnvelope>
              <input type="email" placeholder='Enter Your Email Id' required className='input emaill' />
              <button type='submit' className='footer-button'><FaArrowRight className='FaArrowRight'></FaArrowRight></button>
            </form>
            <div className="social-icons">
            <FaPinterestP className='FaPinterestP'></FaPinterestP>
            <FaFacebook className='FaFacebook'></FaFacebook>
            <FaInstagram className='FaInstagram'></FaInstagram>
            <FaTwitter className='FaTwitter'></FaTwitter>
            </div>
          </div> 
            
            </div>  
            <hr />
            <p className='text-center'>website @2022-All Rights Reserved</p> 
         </footer>
    </div>
  )
}

export default Footer