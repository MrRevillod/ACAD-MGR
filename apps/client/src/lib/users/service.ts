import { User } from "./entity"
import { http } from "$lib/shared/http/client"

import type { CreateUserDTO, UpdateUserDTO, UserDTO } from "./dtos"

class UsersService {
	public list(params?: { search?: string; role?: string }): Promise<User[]> {
		const users = http.request<UserDTO[]>({
			method: "GET",
			url: "/users",
			params,
		})

		return users.then((users) => users.map(User.fromDTO))
	}

	public get(id: string): Promise<User> {
		return http.request<User>({
			method: "GET",
			url: `/users/${id}`,
		})
	}

	public create(data: CreateUserDTO): Promise<User> {
		return http.request<User>({
			method: "POST",
			url: "/users",
			data,
		})
	}

	public update(id: string, data: UpdateUserDTO): Promise<User> {
		return http.request<User>({
			method: "PUT",
			url: `/users/${id}`,
			data,
		})
	}

	public delete(id: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/users/${id}`,
		})
	}
}

export const usersService = new UsersService()
