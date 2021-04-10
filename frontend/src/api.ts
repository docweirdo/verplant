export interface Course {
  id: number;
  name: string;
  default_duration?: number;
  default_room_id?: number;
  course_type?: string;
}

export enum AppointmentStatus {
  /// suggested to me
  Suggested = "SUGGESTED", 
  Approved = "APPROVED", 
  Rejected = "REJECTED", 
  /// suggested by me
  Pending = "PENDING"
}

export interface Appointment {
  id: number;
  starttime: Date;
  endtime: Date;
  status: AppointmentStatus;
  proposer_id: number;
}


interface Api {
  getCourses(): Promise<Course[]>;
  getAppointments(bookingURL: string): Promise<Appointment[]>; 
}

class FakeApi implements Api {
  getCourses() {
    return Promise.resolve([
      {
        id: 1,
        name: "Photoshop Kreativkurs Edition 2000",
        course_type: "Digital",
      },
      {
        id: 2,
        name: "CAD",
        course_type: "Digital",
      },
      {
        id: 3,
        name: "Aktzeichnen",
        course_type: "Zeichnen",
      },
      {
        id: 4,
        name: "Kartoffeldruck",
        course_type: "Zeichnen",
      },
    ]);
  }
  
  getAppointments(bookingURL: string): Promise<Appointment[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve([
          {
            id: 1,
            starttime: new Date('2021-02-01T14:32Z'),
            endtime: new Date('2021-02-01T15:32Z'), 
            status: AppointmentStatus.Rejected,
            proposer_id: 1
          },/* 
          {
            id: 2,
            starttime: new Date('2021-05-01T14:32Z'),
            endtime: new Date('2021-05-01T14:12Z'),
            status: AppointmentStatus.Pending,
            proposer_id: 1
          }, */
          {
            id: 3,
            starttime: new Date('2021-05-02T14:32Z'),
            endtime: new Date('2021-05-02T14:12Z'),
            status: AppointmentStatus.Suggested,
            proposer_id: 2
          },
          {
            id: 4,
            starttime: new Date('2021-05-04T14:32Z'),
            endtime: new Date('2021-05-04T14:12Z'),
            status: AppointmentStatus.Approved,
            proposer_id: 2
          }
        ]);
      }, 1000);
    });
  }
}

class HttpApi implements Api {
  async getCourses(): Promise<Course[]> {
    const result = await fetch("/api/courses", { credentials: "include" });
    const obj = await result.json();
    if (!Array.isArray(obj)) {
      console.warn("Expected array");
    }
    return obj;
  }

  async getAppointments(bookingURL : string): Promise<Appointment[]> {
    const result = await fetch(`/api/bookings/${bookingURL}/appointments`, { credentials: "include" });
    const obj = await result.json() as any[];
    if (!Array.isArray(obj)) {
      console.warn("Expected array");
    }
    return obj.map(appointment => {
      return {
        ...appointment,
        starttime: new Date(appointment.starttime),
        endtime: new Date(appointment.endtime),
      } as Appointment;
    });
  }
}

export const api: Api = new FakeApi();
//export const api: Api = new HttpApi();
