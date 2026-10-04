/** Internal type. DO NOT USE DIRECTLY. */
type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
/** Internal type. DO NOT USE DIRECTLY. */
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
import { DocumentTypeDecoration } from '@graphql-typed-document-node/core';
import { useQuery, UseQueryOptions } from '@tanstack/react-query';
import { fetcher } from '@/app/lib/graphql-client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  NaiveDate: { input: unknown; output: unknown; }
};

export type Category = {
  __typename?: 'Category';
  createdAt: Scalars['NaiveDate']['output'];
  id: Scalars['Int']['output'];
  name: Scalars['String']['output'];
  subCategories: Array<Renamed>;
};

export type CreateUserInput = {
  email: Scalars['String']['input'];
  name: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

export type Mutation = {
  __typename?: 'Mutation';
  /**
   * create user(TODO: hash password)
   * example:
   *
   * mutation {
   * createUser(input: { name: "hanako", email: "hanako@example.com", password: "pass5678" })
   * }
   */
  createUser: Scalars['Boolean']['output'];
  /**
   * create users(TODO: hash password)
   * example:
   *
   * mutation {
   * createUsers(inputs: [
   * { name: "hanako", email: "hanako_multi@example.com", password: "pass5678" },
   * { name: "jiro", email: "jiro_multi@example.com", password: "pass1234" }
   * ]) {
   * id
   * name
   * email
   * password
   * created_at
   * updated_at
   * }
   * }
   */
  createUsers: Array<User>;
};


export type MutationCreateUserArgs = {
  input: CreateUserInput;
};


export type MutationCreateUsersArgs = {
  inputs: Array<CreateUserInput>;
};

export type Query = {
  __typename?: 'Query';
  categories: Array<Category>;
  now: Scalars['String']['output'];
  totalPhotos: Scalars['Int']['output'];
  users: Array<User>;
};

export type Renamed = {
  __typename?: 'Renamed';
  name: Scalars['String']['output'];
};

export type User = {
  __typename?: 'User';
  createdAt: Scalars['String']['output'];
  email: Scalars['String']['output'];
  id: Scalars['Int']['output'];
  name: Scalars['String']['output'];
  password: Scalars['String']['output'];
  updatedAt: Scalars['String']['output'];
};

export type DashboardQueryVariables = Exact<{ [key: string]: never; }>;


export type DashboardQuery = { now: string, totalPhotos: number };


export class TypedDocumentString<TResult, TVariables>
  extends String
  implements DocumentTypeDecoration<TResult, TVariables>
{
  __apiType?: NonNullable<DocumentTypeDecoration<TResult, TVariables>['__apiType']>;
  private value: string;
  public __meta__?: Record<string, any> | undefined;

  constructor(value: string, __meta__?: Record<string, any> | undefined) {
    super(value);
    this.value = value;
    this.__meta__ = __meta__;
  }

  override toString(): string & DocumentTypeDecoration<TResult, TVariables> {
    return this.value;
  }
}

export const DashboardDocument = new TypedDocumentString(`
    query Dashboard {
  now
  totalPhotos
}
    `);

export const useDashboardQuery = <
      TData = DashboardQuery,
      TError = Error
    >(
      variables?: DashboardQueryVariables,
      options?: Omit<UseQueryOptions<DashboardQuery, TError, TData>, 'queryKey'> & { queryKey?: UseQueryOptions<DashboardQuery, TError, TData>['queryKey'] }
    ) => {
    
    return useQuery<DashboardQuery, TError, TData>(
      {
    queryKey: variables === undefined ? ['Dashboard'] : ['Dashboard', variables],
    queryFn: fetcher<DashboardQuery, DashboardQueryVariables>(DashboardDocument, variables),
    ...options
  }
    )};
