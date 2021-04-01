export interface Course {
  id: number;
  name: string;
  default_duration?: number;
  default_room_id?: number;
  course_type?: string;
}

interface Api {
  getCourses(): Promise<Course[]>;
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
}

class HttpApi implements Api {
  async getCourses(): Promise<Course[]> {
    const result = await fetch("/api/courses");
    return result.json();
  }
}

export const api: Api = new FakeApi();
//export const api: Api = new HttpApi();
