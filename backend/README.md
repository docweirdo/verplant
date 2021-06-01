# Verplant backend

## common debugging commands


### authenticated
`
let bookingAddRes = await fetch('/api/bookings/id/1234', { method: 'POST'})
`


### login
`
let loginRes = await fetch('/api/auth/login', { method: 'POST', body: JSON.stringify({password: '', })})
`