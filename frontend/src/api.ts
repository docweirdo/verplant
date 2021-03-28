
export interface Course {
    id: number,
    name: string,
    default_duration?: number,
    default_room_id?: number,
    course_type?: string
  }
  

export async function getCourses(): Promise<Course[]>{
    const result = await fetch("/api/courses");
    return result.json();
}