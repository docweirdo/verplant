# verplant

An in the works fusion of doodle, appointment negotiation and resource planning.

### Techstack

This application is built using a Rust backend HTTP server with the Rocket crate and a VUE 3 Frontend with the PrimeVue Component Library.
For developement we'll be using an SQLITE Database, however in production a switch to MySQL is planned.

### Functionality

The goal for this application is to bring together a bunch of functionalities:
Course Booking, Appointment negotation between provider and customer and simultaneously keeping track of rooms to meet in.

A group of providers provide courses or services to customers, who can book these
services from a starting page by typing in their contact information, the desired course/service and optionally make suggestions for time and date.
They'll be given a static link which they can later on use to check on and interact with their booking process. A provider will be able to log into
their account and check suggested dates as well as make suggestions of their own. They will be able to see the suggested appointments of their customers
as well as the suggested appointments of other booking processes concerning courses/services which are associated with the same room as the courses/services
they provide, so they can spot conflicting suggestions and make sure no appointments are made at a time when a room needed to perform the course/service is not available.

### Cool looking graphs

We laid out or data model in an [Entity Relationship Model](./erm.png).

### Non-goals

At the moment we do **not** aim for these features:

- appointment negotiation between parties of more than two ([Croodle](https://github.com/jelhan/croodle) or [FramaDate](https://framagit.org/framasoft/framadate/framadate))
- appointment scheduling without dedicated confirmation of appointments by both sides ([Easy!Appointments](https://easyappointments.org/))
- dedicated room booking and/or ressource management ([Booked Scheduler](https://github.com/effgarces/BookedScheduler))
- pretty much anything not needed for an MVP with the features described above in _Functionality_

The name of the project is german for: someone or something being planned (out) or scheduled (out), but also somenone being chaotic, confused or unorganized

### TODO

- Fix/Streamline Mobile Layout
- Think about empty space on ContactInformation
- Build fake history when accessing booking URL directly for back button
- Check out wether transactions are necessary everywhere
- Backend: set cookie same-site attribute to strict/lax
