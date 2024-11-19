/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    darkMode: "selector",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        extend: {
            animation: {
                fadeIn: 'fadeIn 1s forwards',
                slideUp: 'slideUp 1.5s forwards',
            },
            keyframes: {
                fadeIn: {
                    '0%': {opacity: 0},
                    '100%': {opacity: 1},
                },
                slideUp: {
                    '0%': {transform: 'translateY(50px)', opacity: 0},
                    '100%': {transform: 'translateY(0)', opacity: 1},
                },
            },
        },
        fontFamily: {
            amsterdam: ["NewAmsterdam-Regular", "amsterdam"],
            strait: ["Strait-Regular", "strait"],
        },
    },
    plugins: [],
};
