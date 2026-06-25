export const queryKeys = {
	university: {
		all: ["university"] as const,
		faculties: {
			all: ["university", "faculties"] as const,
			list: () => [...queryKeys.university.faculties.all, "list"] as const,
			byId: (id: string) => [...queryKeys.university.faculties.all, id] as const,
		},
		departments: {
			all: ["university", "departments"] as const,
			list: () => [...queryKeys.university.departments.all, "list"] as const,
			byFaculty: (facultyId: string) =>
				[...queryKeys.university.departments.all, "byFaculty", facultyId] as const,
			byId: (id: string) => [...queryKeys.university.departments.all, id] as const,
		},
		careers: {
			all: ["university", "careers"] as const,
			list: () => [...queryKeys.university.careers.all, "list"] as const,
			byDepartment: (departmentId: string) =>
				[...queryKeys.university.careers.all, "byDepartment", departmentId] as const,
			byId: (id: string) => [...queryKeys.university.careers.all, id] as const,
		},
		workPositions: {
			all: ["university", "workPositions"] as const,
			list: () => [...queryKeys.university.workPositions.all, "list"] as const,
			byId: (id: string) => [...queryKeys.university.workPositions.all, id] as const,
		},
	},
	academic: {
		all: ["academic"] as const,
		categories: {
			all: ["academic", "categories"] as const,
			list: () => [...queryKeys.academic.categories.all, "list"] as const,
			byId: (id: string) => [...queryKeys.academic.categories.all, id] as const,
		},
		categoryOptions: {
			all: ["academic", "categoryOptions"] as const,
			list: () => [...queryKeys.academic.categoryOptions.all, "list"] as const,
			byCategory: (categoryId: string) =>
				[...queryKeys.academic.categoryOptions.all, "byCategory", categoryId] as const,
			byId: (id: string) =>
				[...queryKeys.academic.categoryOptions.all, id] as const,
		},
		degrees: {
			all: ["academic", "degrees"] as const,
			list: () => [...queryKeys.academic.degrees.all, "list"] as const,
			byId: (id: string) => [...queryKeys.academic.degrees.all, id] as const,
		},
		academics: {
			all: ["academic", "academics"] as const,
			list: () => [...queryKeys.academic.academics.all, "list"] as const,
			byId: (id: string) => [...queryKeys.academic.academics.all, id] as const,
		},
	},
}
