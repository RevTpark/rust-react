import React from 'react'
import Card from 'react-bootstrap/Card';
import Button from 'react-bootstrap/Button';
import prod1 from "../Mainpage/images/prod1.jpg"
import prod2 from '../Mainpage/images/prod2.jpg'
import prod3 from '../Mainpage/images/prod3.jpg'
import './MainSection.css'

function Products() {
   
  return (
    <div>
        <div className="container py-5">
            <div className="row xs={1} md={2}">
               <div className='col-12 text-center'>
                <h1>PRODUCTS</h1>
                <hr />
               </div>
            </div>
        </div>
        <div className="container">
                <div className="row">
                    <div className='col xs lg="2"'>
                <Card style={{ width: '18rem' }}>
            <Card.Img variant="top" src={prod1}  height = {400}/>
            <Card.Body>
              <Card.Title>Product 1</Card.Title>
              <Card.Text>
                Some quick example text to build on the card title and make up the
                bulk of the card's content.
              </Card.Text>
              <Button variant="primary">BUY</Button>
            </Card.Body>
          </Card>
          </div>
                    <div className='col md="auto"'>
                <Card style={{ width: '18rem' }}>
            <Card.Img variant="top" src={prod2} height = {400} />
            <Card.Body>
              <Card.Title>Product 2</Card.Title>
              <Card.Text>
                Some quick example text to build on the card title and make up the
                bulk of the card's content.
              </Card.Text>
              <Button variant="primary">BUY</Button>
            </Card.Body>
          </Card>
          </div>
                    <div className='col xs lg="2"'>
                <Card style={{ width: '18rem' }}>
            <Card.Img variant="top" src={prod3} height = {400} />
            <Card.Body>
              <Card.Title>Product 3</Card.Title>
              <Card.Text>
                Some quick example text to build on the card title and make up the
                bulk of the card's content.
              </Card.Text>
              <Button variant="primary">BUY</Button>
            </Card.Body>
          </Card>
          </div>
                </div>

        </div>
    </div>
  )
}

export default Products