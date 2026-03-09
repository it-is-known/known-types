# This is free and unencumbered software released into the public domain.

require 'json'
require 'yaml'

require 'hashie' # https://rubygems.org/gems/hashie

require_relative 'openapi_flattener'
require_relative 'openapi/transforms'

module OpenAPI
  def self.load_file(path)
    Hashie.symbolize_keys!(YAML.load_file(path, aliases: true))
  end

  def self.type_of(schema)
    case
      when schema[:'$ref'] then :ref
      when schema[:oneOf] then :one_of
      when schema[:anyOf] then :any_of
      when schema[:allOf] then :all_of
      when schema[:properties] || schema[:type] == 'object' then :object
      when schema[:items] || schema[:type] == 'array' then :array
      when schema[:type] then schema[:type]
      else raise ArgumentError, schema.inspect
    end
  end
end # OpenAPI
