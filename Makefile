##
## EPITECH PROJECT, 2022
## Rustracer
## File description:
## Makefile
##

NAME	=	raytracer

all: $(NAME)

$(NAME):
	cargo build --release
	cp target/release/$(NAME) .

clean:
	$(RM) $(NAME)

fclean: clean
	cargo clean

re: fclean all

style:
	cargo clippy

.PHONY: all clean fclean re style
