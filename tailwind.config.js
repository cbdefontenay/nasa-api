/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    darkMode: 'selector',
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        extend: {
            animation: {
                fadeIn: 'fadeIn 1s ease-out',
                slideIn: 'slideIn 0.5s ease-out',
            },
            keyframes: {
                fadeIn: {
                    '0%': {opacity: '0'},
                    '100%': {opacity: '1'},
                },
                slideIn: {
                    '0%': {transform: 'translateX(-10%)', opacity: '0'},
                    '100%': {transform: 'translateX(0)', opacity: '1'},
                },
            },
        },
        fontFamily: {
            "amsterdam": ["NewAmsterdam-Regular", "amsterdam"],
            "strait": ["Strait-Regular", "strait"],
        }
    },
    plugins: [],
};
