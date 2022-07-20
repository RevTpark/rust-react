import React , {useEffect, useState} from 'react'
import {Link} from 'react-router-dom'
import {FaBars, FaTimes} from 'react-icons/fa'
import {MdFingerprint} from 'react-icons/md'
import { click } from '@testing-library/user-event/dist/click'
import { Button } from './Button '
import './Navbar.css'
import {IconContext } from 'react-icons/lib'


function Navbar() {
    const [click, setClick] = useState(false)
    const[button, Setbutton] = useState(true)
    const handleClick = () => setClick(!click)
    const closeMobileMenu = () => setClick(false)

    const showButton = () =>{
        if(window.innerWidth<=960){
            Setbutton(false)
        }
        else{
            Setbutton(true)
        }
    };
     useEffect( () =>{
         showButton();
     }, []);
    window.addEventListener('resize', showButton)
    return (
        <>
        <IconContext.Provider value ={{color: '#000'}}>
         <div className="Navbar">
             
             <div className="Navbar-container container">
                <Link to = '/' className = 'Navbar-Logo' onClick={closeMobileMenu}>
                    <MdFingerprint className='Navbar-Icon'/>
                    WELCOME
                </Link>
                <div className="menu-icon" onClick={handleClick}>
                {click? <FaTimes classname = 'fa-Times'/> : <FaBars className='fa-Bars'/>}
                </div>
                <ul className={click ? 'nav-menu active' : 'nav-menu'}>
                    <li className='nav-item'>
                        <Link to = '/' className="nav-links" onClick={closeMobileMenu}>
                            Home
                        </Link>
                    </li>
                    <li className='nav-item'>
                        <Link to = '/Services' className="nav-links" onClick={closeMobileMenu}>
                            About Us
                        </Link>
                    </li>
                    <li className='nav-item'>
                        <Link to = '/Products' className="nav-links" onClick={closeMobileMenu}>
                           Products
                        </Link>
                    </li>
                    <li className='nav-btn'>
                        {button ? (
                            <Link  to = '/MultipleInputs'className="btn-link">
                                <Button buttonStyle = 'btn--outline'>
                                    SIGN UP
                                </Button>
                            </Link>
                        ):( 
                            <Link to = '/MultipleInputs' className="btn-link" onClick={closeMobileMenu}>
                                <Button buttonStyle ='btn--outline' buttonSize = 'btn--mobile'> SIGNUP </Button>
                            </Link>
                        )

                        }

                    </li>


                </ul>
             </div>
             
             </div>  
             </IconContext.Provider> 
        </>
    )
}

export default Navbar
                     