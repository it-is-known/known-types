# This is free and unencumbered software released into the public domain.

module OpenAPI; end
module OpenAPI::Transforms; end

##
# Fold and eliminate useless `$ref` aliases, often the result of synthetic
# schemas created by preceding transformations.
#
# This transformation is idempotent.
module OpenAPI::Transforms::DerefAliases
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schemas!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    aliases = schemas.inject({}) do |result, (schema_ref, schema)|
      result[schema_ref] = schema if schema[:'$ref']
      result
    end
    schemas.each_value do |schema|
      transform_schema!(schema, schemas, aliases)
    end
    aliases.keys.each { schemas.delete(it) }
  end # transform_schemas!

  protected

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schema!(schema, schemas, aliases = {})
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:'$ref']
        ref = schema[:'$ref'].split('/').last.to_sym
        if target = aliases[ref]
          schema.merge!(target)
        end
      when schema[:oneOf], schema[:anyOf]
        variants = schema[:oneOf] || schema[:anyOf] || []
        variants.each do |variant|
          transform_schema!(variant, schemas, aliases)
        end
      when schema[:allOf]
        variants = schema[:allOf] || []
        variants.each { transform_schema!(it, schemas, aliases) }
      when schema[:properties] || schema[:type] == 'object'
        properties = schema[:properties] || {}
        properties.each_value do |property|
          transform_schema!(property, schemas, aliases)
        end
      when schema[:items] || schema[:type] == 'array'
        transform_schema!(schema[:items], schemas, aliases) if schema[:items]
      else # nothing to do
    end
  end # transform_schema!
end # OpenAPI::Transforms::DerefAliases
