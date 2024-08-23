const validators = {
    email: /^[\w-.]{2,32}@([\w-]{2,32}.)+[\w-]{2,4}$/,
    password: /^(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*\W)(?!.* ).{8,16}$/
};

const validate = (credentials: any): boolean => {
    return Object.keys(validators).every((key) =>
        validators[key as keyof typeof validators].test(credentials[key])
    );
};

const send = (path: "register" | "login" | "verify", body: any): Promise<any> => {
    return $fetch(`/api/auth/${path}`, {
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(body)
    });
};

export const useRegister = async (credentials: any) => {
    if (!validate(credentials)) {
        return;
    }

    const data = await send("register", credentials);

    if (!data.error) {
        return navigateTo("/auth/login");
    }

    return data;
};

export const useLogin = async (credentials: any) => {
    if (!validate(credentials)) {
        return;
    }

    const data = await send("login", credentials);

    if (!data.error) {
        return navigateTo("/dashboard");
    }

    return data;
};