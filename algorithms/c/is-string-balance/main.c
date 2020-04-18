#include <stdio.h>
#include <string.h>
#include <stdlib.h>

struct Node
{
  char value;
  struct Node *previous;
  struct Node *next;
};

struct List
{
  unsigned int length;
  struct Node *head;
};

struct Node *
makeNode(char value)
{
  struct Node *node = malloc(sizeof(struct Node));
  node->value = value;
  return node;
}

struct List *makeList()
{
  struct List *list = malloc(sizeof(struct List));
  list->length = 0;
  list->head = NULL;
  return list;
}

int isEmpty(struct List *list)
{
  return list->length == 0;
}

void append(struct List *list, char value)
{
  list->length += 1;
  if (list->head == NULL)
  {
    list->head = makeNode(value);
    return;
  }
  struct Node *currentNode = list->head;
  for (int i = 0; i < list->length - 1; i += 1)
  {
    currentNode = currentNode->next;
  }
  currentNode->next = makeNode(value);
}

char pop(struct List *list)
{
  if (list->length == 1)
  {
    char value = list->head->value;
    list->length -= 1;
    list->head = NULL;
    return value;
  }
  struct Node *currentNode = list->head;
  for (int i = 0; i < list->length - 1; i += 1)
  {
    currentNode = currentNode->next;
  }

  char value = currentNode->value;
  currentNode = NULL;
  list->length -= 1;
  return value;
}

int isOpeningCharacter(char character)
{
  return character == '(' ||
         character == '[' ||
         character == '{';
}

int isMatch(char left, char right)
{
  if (left == '(')
  {
    return right == ')';
  }
  if (left == '[')
  {
    return right == ']';
  }
  if (left == '{')
  {
    return right == '}';
  }

  return 0;
}

int isBalanced(char *string)
{

  struct List *list = makeList();

  unsigned int length = strlen(string);

  for (int i = 0; i < length; i += 1)
  {
    if (isOpeningCharacter(string[i]))
    {
      append(list, string[i]);
    }
    else if (isEmpty(list) || !isMatch(pop(list), string[i]))
    {
      return 0;
    }
  }

  return isEmpty(list);
}

int main()
{
  /*
  Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

  An input string is valid if :

  Open brackets must be closed by the same type of brackets.Open brackets must be closed in the correct order.

  Note that an empty string is also considered valid.

  Examples:

  Input : "()" Output : true
  Input : "()[]{}" Output : true
  Input : "(]" Output : false
*/

  printf("() -> %d\n", isBalanced("()"));
  printf("()[]{} -> %d\n", isBalanced("()[]{}"));
  printf("(] -> %d\n", isBalanced("(]"));
  return 0;
}
