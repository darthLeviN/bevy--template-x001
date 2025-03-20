# Bevy Game Template

This is a template to help you get started with making a game using Bevy. While the template is functional, it's still
missing some key features.

## What's Implemented So Far

- **Event Handler**: A system that catches events not handled by any other observer.
- **Scene System**: A generic scene system built on top of Bevy's native scene system, with special handling for
  elements like UI scenes.
- **UI Page Navigation**: Enables navigation between named page scenes using a path-based approach.
- **Focus Handling**: Custom focus management, allowing input propagation through focused UI nodes. *(Note: Will be
  rewritten for Bevy 0.16.)*
- **Interaction Styling**: Supports styling based on interaction states (hover, press, focus, etc.).
- **Theme System**: Allows the use of a stylesheet for bundled interaction styles. Themes can be stored as a global
  resource or as `Theme` components. Child components will inherit the theme automatically.

## Next Steps

Here are the next features to be implemented:

- **Text Input Handler**: Add a handler for managing text input.
- **App Configuration/User Storage**: Set up a system for app configurations and user data storage.
- **Efficient Game Asset Management**: Add functionality to handle game assets efficiently.
- **Prepared UI Controls**: Create reusable and styled UI controls, such as buttons.


## Future plans
In the future i will split this project into different parts. A widget library will be a major part.
