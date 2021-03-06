#pragma once

#include "macros.h"

namespace Alkahest
{
    /**
     * Used as the application entrypoint. When creating an application
     * using Alkahest, the developer should define three functions:
     * * Alkahest::Game::init()
     * * Alkahest::Game::update()
     * * Alkahest::Game::cleanup()
     *
     * These three functions define the client-side behaviour of the
     * application so that it can be customized.
     */
    class API Game
    {
    public:
        Game() {};
        ~Game() {};
    public:
        /** The initialization function for the client application.
         *
         * This function should be defined by the developer and should
         * initialize any resources required be the application.
         */
        void init();

        /** The update function for the client application.
         *
         * This function should be defined by the developer and should
         * update any values that should change for the current iteration of
         * the game loop.
         */
        void update();

        /** The cleanup function for the client application.
         *
         * This function should be defined by the developer and should
         * clean up any resources that were initialized over the course of
         * the game life.
         */
        void cleanup();
    public:
        void sysInit();
        void sysUpdate();
        void sysCleanup();
    public:
        /** The engine run loop */
        void run();
    public:
        static void stop();
    };
}
